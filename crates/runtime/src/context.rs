use anyhow::Result;
use rama::http::Request as RamaRequest;
use std::str::FromStr;
use wasmtime::component::ResourceTable;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

use super::bindings::{Host, Request};

pub struct Context {
    pub wasi: WasiCtx,
    pub table: ResourceTable,
    pub request: RamaRequest,
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
    pub fn new(request: RamaRequest) -> Self {
        Context {
            wasi: WasiCtxBuilder::new().inherit_stdio().inherit_args().build(),
            table: ResourceTable::new(),
            request,
        }
    }
}

impl Host for Context {}

impl Request for Context {
    fn headers(&mut self) -> Vec<(String, String)> {
        self.request
            .headers()
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_str().unwrap().to_string()))
            .collect()
    }

    fn set_header(&mut self, key: String, value: String) -> Result<(), String> {
        let key = http::HeaderName::from_bytes(key.as_bytes()).map_err(|e| e.to_string())?;
        let value = http::HeaderValue::from_bytes(value.as_bytes()).map_err(|e| e.to_string())?;
        self.request
            .headers_mut()
            .insert(key, value)
            .map(|_| ())
            .ok_or_else(|| "Failed to insert header".to_string())
    }

    fn uri(&mut self) -> String {
        self.request.uri().to_string()
    }

    fn set_uri(&mut self, uri: String) -> Result<(), String> {
        http::Uri::from_str(&uri)
            .map(|u| *self.request.uri_mut() = u)
            .map_err(|e| format!("Could not create uri {}: {}", uri, e.to_string()))
    }
}
