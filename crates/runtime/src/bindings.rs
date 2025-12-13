use wasmtime::component::{HasSelf, Linker, bindgen};

use super::context::Context;

bindgen!({
    path: "../../crates/proxy/wit",
    world: "crossroads",
});

pub(crate) fn add_to_linker(linker: &mut Linker<Context>) -> Result<(), anyhow::Error> {
    wit::crossroads::types::add_to_linker::<_, HasSelf<_>>(linker, |state| state)?;
    wit::crossroads::request::add_to_linker::<_, HasSelf<_>>(linker, |state| state)
}

pub(crate) use wit::crossroads::request::Host as Request;
pub(crate) use wit::crossroads::types::{Host, Resolution, Response};
