use rama::http::client::EasyHttpWebClient;
use rama::http::{Body, Request, Response, StatusCode};
use rama::{Context, Service};
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::oneshot;

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
        let (callback, or) = oneshot::channel::<Result<Request, Response<Body>>>();

        let message = Message::ProcessRequest { request, callback };
        match self.sender.send(message) {
            Ok(_) => (),
            Err(_) => {
                return Ok(Response::builder()
                    .status(StatusCode::BAD_GATEWAY)
                    .body("Failed to send to WebAssembly Runtime".into())
                    .unwrap())
            }
        }

        match or.await.unwrap() {
            Ok(request) => {
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
            Err(response) => Ok(response),
        }
    }
}
