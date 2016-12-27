extern crate tokio_line_example;
extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

use tokio_line_example::LineClientProto;
use tokio_proto::TcpClient;
use futures::Future;
use tokio_service::Service;

fn main() {
    let mut event_loop = tokio_core::reactor::Core::new().unwrap();
    let handle = event_loop.handle();

    let addr = "0.0.0.0:12345".parse().unwrap();
    let test = TcpClient::new(LineClientProto)
        .connect(&addr, &handle.clone())
        .and_then(|mut client| {
            let req = "hello".into();
            println!("req: {:?}", req);
            client.call(req)
        })
        .map(|res| println!("res: {:?}", res));

    event_loop.run(test).unwrap();
}
