use anyhow::{Result, anyhow};
use rama::http::{HeaderName, HeaderValue, Uri};
use std::collections::HashMap;
use wasmtime::component::{Resource, ResourceTable, TypedFunc, bindgen};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

use crate::Request;
use crate::bindings::{Host, HostRequest};

pub struct Context {
    pub wasi: WasiCtx,
    pub table: ResourceTable,
    pub requests: HashMap<u32, rama::http::Request>,
}

impl WasiView for Context {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi,
            table: &mut self.table,
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Context {
            wasi: WasiCtxBuilder::new().inherit_stdio().inherit_args().build(),
            table: ResourceTable::new(),
            requests: HashMap::new(),
        }
    }
}

impl Host for Context {}

impl HostRequest for Context {
    fn headers(&mut self, self_: Resource<Request>) -> Result<Vec<(String, String)>, String> {
        let request = self
            .requests
            .get(&self_.rep())
            .ok_or_else(|| "Request not in resource table".to_string())?;
        let header = request
            .headers()
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_str().unwrap().to_string()))
            .collect();
        Ok(header)
    }

    fn set_header(
        &mut self,
        self_: Resource<Request>,
        key: String,
        value: String,
    ) -> Result<(), String> {
        let header_key = HeaderName::from_bytes(key.as_bytes()).map_err(|err| err.to_string())?;
        let header_value = HeaderValue::from_str(&value).map_err(|err| err.to_string())?;
        self.requests
            .get_mut(&self_.rep())
            .ok_or_else(|| "Request not in resource table".to_string())?
            .headers_mut()
            .insert(header_key, header_value);
        Ok(())
    }

    fn uri(&mut self, self_: Resource<Request>) -> Result<String, String> {
        let request = self
            .requests
            .get(&self_.rep())
            .ok_or_else(|| "Request not in resource table".to_string())?;
        Ok(request.uri().to_string())
    }

    fn set_uri(&mut self, self_: Resource<Request>, uri: String) -> Result<(), String> {
        let uri = Uri::from_maybe_shared(uri)
            .map_err(|err| format!("Error assigning uri: {}", err.to_string()))?;
        let request = self
            .requests
            .get_mut(&self_.rep())
            .ok_or_else(|| "Request not in resource table".to_string())?;
        *request.uri_mut() = uri;
        Ok(())
    }

    fn drop(&mut self, self_: Resource<Request>) -> Result<()> {
        self.table
            .delete(self_)
            .map_err(|err| anyhow!("Error dropping resource: {}", err.to_string()))
    }
}
