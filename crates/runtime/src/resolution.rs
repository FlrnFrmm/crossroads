#[derive(Debug)]
pub enum Resolution {
    Forward(rama::http::Request),
    Respond(rama::http::Response),
}
