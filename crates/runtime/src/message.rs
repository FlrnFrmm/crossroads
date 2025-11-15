use rama::http::Request;
use tokio::sync::oneshot::{self, Receiver, Sender};

use crate::proxy::Proxy;
use crate::resolution::Resolution;

pub enum Message {
    SetComponent(Proxy),
    ProcessRequest {
        request: Request,
        callback: Sender<Resolution>,
    },
}
impl Message {
    pub fn new_process_request(request: Request) -> (Self, Receiver<Resolution>) {
        let (sender, receiver) = oneshot::channel::<Resolution>();
        let message = Self::ProcessRequest {
            request,
            callback: sender,
        };
        (message, receiver)
    }
}
