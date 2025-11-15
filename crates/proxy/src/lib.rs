#[allow(warnings)]
mod bindings;

use bindings::exports::wit::crossroads::proxy::Guest;
use bindings::wit::crossroads::types::{Request, Resolution, Response};

struct Component;

impl Guest for Component {
    fn handle(_: Request) -> Resolution {
        let response = Response {
            status_code: 404,
            body: None,
        };
        Resolution::Respond(response)
    }
}

bindings::export!(Component with_types_in bindings);
