use rama::http::{Body, Request, Response};
use tokio::sync::oneshot::Sender;

use crate::proxy::Proxy;

pub enum Message {
    SetComponent(Proxy),
    ProcessRequest {
        request: Request,
        callback: Sender<Result<Request, Response<Body>>>,
    },
}
