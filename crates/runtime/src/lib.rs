mod bindings;
mod context;
mod message;
pub mod proxy;

use anyhow::{Result, anyhow};
use rama::http::{Request as RamaRequest, Response, StatusCode};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use wasmtime::component::{Component, Linker, Resource, TypedFunc};
use wasmtime::{Engine, Store};

use context::Context;
pub use message::Message;

const DEFAULT_PROXY_LENGTH: usize = include_bytes!("../../default-proxy/proxy.wasm").len();
const DEFAULT_PROXY: [u8; DEFAULT_PROXY_LENGTH] = *include_bytes!("../../default-proxy/proxy.wasm");

pub type Request = ();
pub type ProxyFunc = TypedFunc<(Resource<Request>,), (Result<(), String>,)>;

pub struct Runtime {
    engine: Engine,
    linker: Linker<Context>,
    store: Store<Context>,
    proxy_func: ProxyFunc,
    sender: UnboundedSender<Message>,
    receiver: UnboundedReceiver<Message>,
}

impl Runtime {
    pub fn new() -> Result<Self> {
        let engine = wasmtime::Engine::default();
        let mut store = Store::new(&engine, Default::default());
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
        bindings::add_to_linker(&mut linker)?;
        let component = Component::from_binary(&engine, &DEFAULT_PROXY)?;
        let proxy_func = Self::extract_proxy_function(&linker, &mut store, component)?;
        let (sender, receiver) = mpsc::unbounded_channel::<Message>();
        let runtime = Self {
            engine,
            store,
            linker,
            proxy_func,
            sender,
            receiver,
        };
        Ok(runtime)
    }

    pub fn sender(&self) -> UnboundedSender<Message> {
        return self.sender.clone();
    }

    fn handle(&mut self, request: RamaRequest) -> Result<RamaRequest> {
        let resource = self.store.data_mut().table.push(())?;
        let resource_id = resource.rep();
        self.store.data_mut().requests.insert(resource_id, request);
        let (result,) = self.proxy_func.call(&mut self.store, (resource,))?;
        result.map_err(|error_message| anyhow!("Component error: {}", error_message))?;
        self.proxy_func.post_return(&mut self.store)?;
        let Some(request) = self.store.data_mut().requests.remove(&resource_id) else {
            anyhow::bail!("Couldn't find request ref cell with id {}", resource_id);
        };
        Ok(request)
    }

    pub async fn start(&mut self) -> Result<()> {
        while let Some(message) = self.receiver.recv().await {
            match message {
                Message::ProcessRequest { request, callback } => match self.handle(request) {
                    Ok(request) => {
                        callback
                            .send(Ok(request))
                            .map_err(|_| anyhow!("Error sending request callback"))?;
                    }
                    Err(_) => {
                        let response = Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(().into())
                            .unwrap();
                        callback
                            .send(Err(response))
                            .map_err(|_| anyhow!("Error sending response callback"))?;
                    }
                },
                Message::SetComponent(proxy) => {
                    let component =
                        Component::from_binary(&self.engine, proxy.component.as_slice())?;
                    let result =
                        Self::extract_proxy_function(&self.linker, &mut self.store, component);
                    match result {
                        Ok(proxy_func) => self.proxy_func = proxy_func,
                        Err(error) => {
                            println!("Could not load proxy: {}", error)
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn extract_proxy_function(
        linker: &Linker<Context>,
        store: &mut Store<Context>,
        component: Component,
    ) -> Result<ProxyFunc> {
        let instance = linker.instantiate(&mut *store, &component)?;

        let interface_namespace = "wit:crossroads/router@0.1.0";
        let interface_idx = instance
            .get_export_index(&mut *store, None, interface_namespace)
            .expect(&format!("Cannot get `{}` interface", interface_namespace));

        let parent_export_idx = Some(&interface_idx);
        let func_id_handle_request = instance
            .get_export_index(&mut *store, parent_export_idx, "handle")
            .expect(&format!("Cannot get `{}` function", "handle"));

        let func_handle_request = instance
            .get_func(&mut *store, func_id_handle_request)
            .expect("Unreachable since we've got func_idx");

        func_handle_request.typed::<(Resource<Request>,), (Result<(), String>,)>(store)
    }
}
