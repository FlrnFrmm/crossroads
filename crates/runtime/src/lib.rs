mod bindings;
mod context;

use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::path::Path;
use wasmtime::component::{Component, Linker, Resource, TypedFunc};
use wasmtime::{Engine, Store};

use context::Context;

pub type Request = ();
pub type Router = TypedFunc<(Resource<Request>,), (Result<(), String>,)>;

pub struct Runtime {
    engine: Engine,
    linker: Linker<Context>,
    store: Store<Context>,
    instances: HashMap<usize, Router>,
}

impl Runtime {
    pub fn new() -> Result<Self> {
        let engine = wasmtime::Engine::default();
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
        bindings::add_to_linker(&mut linker)?;
        let runtime = Self {
            store: Store::new(&engine, Default::default()),
            engine,
            linker,
            instances: HashMap::new(),
        };
        Ok(runtime)
    }

    pub fn add_instance(&mut self, path_to_component: impl AsRef<Path>) -> Result<usize> {
        let id = self.instances.keys().max().unwrap_or(&0) + 1;

        let component = Component::from_file(&self.engine, path_to_component)?;
        let instance = self.linker.instantiate(&mut self.store, &component)?;

        let interface_namespace = "wit:crossroads/router@0.1.0";
        let interface_idx = instance
            .get_export_index(&mut self.store, None, interface_namespace)
            .expect(&format!("Cannot get `{}` interface", interface_namespace));

        let parent_export_idx = Some(&interface_idx);
        let func_id_handle_request = instance
            .get_export_index(&mut self.store, parent_export_idx, "handle")
            .expect(&format!("Cannot get `{}` function", "handle"));

        let func_handle_request = instance
            .get_func(&mut self.store, func_id_handle_request)
            .expect("Unreachable since we've got func_idx");

        let handle = func_handle_request
            .typed::<(Resource<Request>,), (Result<(), String>,)>(&self.store)?;

        self.instances.insert(id, handle);

        Ok(id)
    }

    pub fn call_handle(
        &mut self,
        id: usize,
        request: rama::http::Request,
    ) -> Result<rama::http::Request> {
        let resource = self.store.data_mut().table.push(())?;
        let resource_id = resource.rep();
        self.store.data_mut().requests.insert(resource_id, request);
        let Some(router) = self.instances.get(&id) else {
            anyhow::bail!("Couldn't find function with id {}", id);
        };
        let (result,) = router.call(&mut self.store, (resource,))?;
        result.map_err(|error_message| anyhow!("Component error: {}", error_message))?;
        router.post_return(&mut self.store)?;
        let Some(rama_request) = self.store.data_mut().requests.remove(&resource_id) else {
            anyhow::bail!("Couldn't find request ref cell with id {}", resource_id);
        };
        Ok(rama_request)
    }
}
