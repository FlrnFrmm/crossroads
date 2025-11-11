use anyhow::Result;
use rama::http::client::EasyHttpWebClient;
use rama::http::{Body, Request, Response, StatusCode};
use rama::{Context, Service};
use runtime::resolution::Resolution;
use tokio::sync::mpsc::UnboundedSender;

use runtime::Message;

pub struct WebAssemblyComponentProxy {
    sender: UnboundedSender<Message>,
}

impl WebAssemblyComponentProxy {
    pub fn new(sender: UnboundedSender<Message>) -> Self {
        Self { sender }
    }
}

impl<State> Service<State, Request> for WebAssemblyComponentProxy
where
    State: Send + Sync + 'static,
{
    type Response = Response;
    type Error = std::convert::Infallible;

    async fn serve(
        &self,
        _context: Context<State>,
        request: Request,
    ) -> Result<Self::Response, Self::Error> {
        let (message, receiver) = Message::new_process_request(request);
        match self.sender.send(message) {
            Ok(_) => (),
            Err(_) => {
                return Ok(Response::builder()
                    .status(StatusCode::BAD_GATEWAY)
                    .body("Failed to send to WebAssembly Runtime".into())
                    .unwrap());
            }
        }
        match receiver.await {
            Ok(resolution) => match resolution {
                Resolution::Forward(request) => {
                    let client = EasyHttpWebClient::default();
                    match client.serve(Context::default(), request).await {
                        Ok(response) => Ok(response),
                        Err(error) => {
                            let error_message =
                                format!("Failed to connect to destination: {}", error.to_string());
                            Ok(Response::builder()
                                .status(StatusCode::BAD_GATEWAY)
                                .body(error_message.into())
                                .unwrap())
                        }
                    }
                }
                Resolution::Respond(response) => Ok(response),
            },
            Err(e) => {
                let response = Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(format!("Internal Server Error: {}", e)))
                    .unwrap();
                Ok(response)
            }
        }
    }
}
