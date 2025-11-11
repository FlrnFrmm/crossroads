mod bindings;
mod context;

use anyhow::Result;
use rama::http::{Body, Request as RamaRequest, Response as RamaResponse};
use wasmtime::component::{Component, Linker, Resource, TypedFunc};
use wasmtime::{Engine, Store};

pub type Request = ();
pub type ProxyFunc = TypedFunc<(Resource<Request>,), (bindings::Resolution,)>;

pub struct Runtime {
    engine: Engine,
    linker: Linker<context::Context>,
    store: Store<context::Context>,
    proxy_func: ProxyFunc,
}

impl Runtime {
    pub fn new(default_proxy: &[u8]) -> Result<Self> {
        let engine = wasmtime::Engine::default();
        let mut store = Store::new(&engine, Default::default());
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
        bindings::add_to_linker(&mut linker)?;
        let component = Component::from_binary(&engine, default_proxy)?;
        let runtime = Self {
            proxy_func: extract_proxy_function(&linker, &mut store, component)?,
            engine,
            store,
            linker,
        };
        Ok(runtime)
    }

    pub fn process(&mut self, request: RamaRequest) -> Result<super::Resolution> {
        let resource = self.store.data_mut().table.push(())?;
        let resource_id = resource.rep();
        self.store.data_mut().requests.insert(resource_id, request);
        let (result,) = self.proxy_func.call(&mut self.store, (resource,))?;
        self.proxy_func.post_return(&mut self.store)?;
        let Some(request) = self.store.data_mut().requests.remove(&resource_id) else {
            anyhow::bail!("Couldn't find request ref cell with id {}", resource_id);
        };
        match result {
            bindings::Resolution::Forward(_) => Ok(super::Resolution::Forward(request)),
            bindings::Resolution::Respond(bindings::Response { status_code, body }) => {
                let body = body.map(|bytes| Body::from(bytes)).unwrap_or(Body::empty());
                let response = RamaResponse::builder().status(status_code).body(body)?;
                Ok(super::Resolution::Respond(response))
            }
        }
    }

    pub fn set_proxy(&mut self, component: &[u8]) -> Result<()> {
        let component = Component::from_binary(&self.engine, component)?;
        let proxy_func = extract_proxy_function(&self.linker, &mut self.store, component)?;
        self.proxy_func = proxy_func;
        Ok(())
    }
}

fn extract_proxy_function(
    linker: &Linker<context::Context>,
    store: &mut Store<context::Context>,
    component: Component,
) -> Result<ProxyFunc> {
    let instance = linker.instantiate(&mut *store, &component)?;

    let interface_namespace = "wit:crossroads/proxy@0.1.0";
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

    func_handle_request.typed::<(Resource<Request>,), (bindings::Resolution,)>(store)
}
