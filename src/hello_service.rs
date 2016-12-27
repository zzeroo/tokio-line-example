use futures;
use futures::future::FutureResult;
use std::{io, str};
use tokio_service::Service;

pub struct HelloWorldService;

impl Service for HelloWorldService {
    type Request = String;
    type Response = String;
    type Error = io::Error;
    type Future = FutureResult<Self::Response, Self::Error>;

    fn call(&mut self, req: Self::Request) -> Self::Future {
        if req.contains('\n') {
            futures::failed(io::Error::new(io::ErrorKind::InvalidInput, "message contains new line"))
        } else {
            let resp = match req.as_str() {
                "hello" => "world".to_string(),
                _ => "idk".to_string(),
            };
            futures::finished(resp)
        }
    }
}
