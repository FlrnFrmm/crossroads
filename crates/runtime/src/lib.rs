mod message;
pub mod proxy;
pub mod resolution;
mod runtime;

use anyhow::Result;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

pub use message::Message;
use resolution::Resolution;
use runtime::Runtime;

const DEFAULT_PROXY_LENGTH: usize =
    include_bytes!("../../proxy/target/wasm32-wasip2/release/proxy.wasm").len();
const DEFAULT_PROXY: [u8; DEFAULT_PROXY_LENGTH] =
    *include_bytes!("../../proxy/target/wasm32-wasip2/release/proxy.wasm");

pub struct Handler {
    pub sender: UnboundedSender<Message>,
    receiver: UnboundedReceiver<Message>,
    runtime: Runtime,
}

impl Handler {
    pub fn new() -> Result<Self> {
        let (sender, receiver) = mpsc::unbounded_channel::<Message>();
        let runtime = Runtime::new(&DEFAULT_PROXY)?;
        let handler = Self {
            sender,
            receiver,
            runtime,
        };
        Ok(handler)
    }

    pub async fn start(&mut self) -> Result<()> {
        while let Some(message) = self.receiver.recv().await {
            match message {
                Message::ProcessRequest { request, callback } => {
                    match self.runtime.process(request) {
                        Ok(resolution) => match callback.send(resolution) {
                            Ok(_) => {}
                            Err(e) => println!("Could not send resolution: {:?}", e),
                        },
                        Err(e) => println!("Error processing rqequest: {}", e),
                    }
                }
                Message::SetComponent(proxy) => match self.runtime.set_proxy(&proxy.component) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Could not load proxy: {}", e)
                    }
                },
            }
        }
        Ok(())
    }
}
