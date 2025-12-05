mod bindings;
mod context;
pub mod proxy;
pub mod resolution;

use anyhow::{Result, anyhow};
use rama::http::{Body, Request as RamaRequest, Response as RamaResponse};
use wasmtime::component::{Component, Linker, TypedFunc};
use wasmtime::{Engine, Store};

use resolution::Resolution;

pub type Request = ();
pub type ProxyFunc = TypedFunc<(), (bindings::Resolution,)>;

use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct Runtime {
    engine: Engine,
    linker: Linker<context::Context>,
    component: Arc<RwLock<Component>>,
}

impl Runtime {
    pub fn new(default_proxy: &[u8]) -> Result<Self> {
        let engine = wasmtime::Engine::default();
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
        bindings::add_to_linker(&mut linker)?;
        let component = Component::from_binary(&engine, default_proxy)?;

        let runtime = Self {
            engine,
            linker,
            component: Arc::new(RwLock::new(component)),
        };
        Ok(runtime)
    }

    pub fn process(&self, request: RamaRequest) -> Result<Resolution> {
        let mut store = Store::new(&self.engine, context::Context::new(request));
        let component = self
            .component
            .read()
            .map_err(|e| anyhow!("Failed to acquire read lock of component: {}", e))?;
        let proxy_func = extract_proxy_function(&self.linker, &mut store, &component)?;
        drop(component);

        let (result,) = proxy_func.call(&mut store, ())?;
        proxy_func.post_return(&mut store)?;

        match result {
            bindings::Resolution::Forward => Ok(Resolution::Forward(store.into_data().request)),
            bindings::Resolution::Respond(bindings::Response { status_code, body }) => {
                let body = body.map(|bytes| Body::from(bytes)).unwrap_or(Body::empty());
                let response = RamaResponse::builder().status(status_code).body(body)?;
                Ok(Resolution::Respond(response))
            }
        }
    }

    pub fn set_proxy(&self, component: &[u8]) -> Result<()> {
        let component = Component::from_binary(&self.engine, component)?;
        let mut lock = self
            .component
            .write()
            .map_err(|e| anyhow!("Failed to acquire write lock to update component: {}", e))?;
        *lock = component;
        Ok(())
    }
}

fn extract_proxy_function(
    linker: &Linker<context::Context>,
    store: &mut Store<context::Context>,
    component: &Component,
) -> Result<ProxyFunc> {
    let instance = linker.instantiate(&mut *store, component)?;

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

    func_handle_request.typed::<(), (bindings::Resolution,)>(store)
}
