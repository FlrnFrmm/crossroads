use anyhow::{Result, anyhow};
use rama::http::Request as RamaRequest;
use std::collections::HashMap;
use std::str::FromStr;
use wasmtime::component::{Resource, ResourceTable};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

use super::bindings::{Host, HostRequest};
use crate::runtime::Request;

pub struct Context {
    pub wasi: WasiCtx,
    pub table: ResourceTable,
    pub requests: HashMap<u32, RamaRequest>,
}

impl WasiView for Context {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi,
            table: &mut self.table,
        }
    }
}

impl Context {
    fn retrieve_request(&self, session_id: u32) -> Result<&RamaRequest, String> {
        let request = self
            .requests
            .get(&session_id)
            .ok_or_else(|| "Request not in resource table".to_string())?;
        Ok(request)
    }

    fn retrieve_request_mut(&mut self, session_id: u32) -> Result<&mut RamaRequest, String> {
        let request = self
            .requests
            .get_mut(&session_id)
            .ok_or_else(|| "Request not in resource table".to_string())?;
        Ok(request)
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
        let request = self.retrieve_request(self_.rep())?;
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
        let key = http::HeaderName::from_bytes(key.as_bytes()).map_err(|e| e.to_string())?;
        let value = http::HeaderValue::from_bytes(value.as_bytes()).map_err(|e| e.to_string())?;
        self.retrieve_request_mut(self_.rep())?
            .headers_mut()
            .insert(key, value)
            .map(|_| ())
            .ok_or_else(|| "Failed to insert header".to_string())
    }

    fn uri(&mut self, self_: Resource<Request>) -> Result<String, String> {
        self.retrieve_request_mut(self_.rep())
            .map(|r| r.uri().to_string())
    }

    fn set_uri(&mut self, self_: Resource<Request>, uri: String) -> Result<(), String> {
        let uri = http::Uri::from_str(&uri)
            .map_err(|e| format!("Could not create uri {}: {}", uri, e.to_string()))?;
        self.retrieve_request_mut(self_.rep())
            .map(|s| *s.uri_mut() = uri)
    }

    fn drop(&mut self, self_: Resource<Request>) -> Result<()> {
        self.table
            .delete(self_)
            .map_err(|err| anyhow!("Error dropping resource: {}", err.to_string()))
    }
}
