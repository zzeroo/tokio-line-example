extern crate tokio_line_example;

use tokio_line_example::*;

fn main() {
    let addr = "0.0.0.0:12345".parse().unwrap();
    TcpServer::new(LineServerProto, addr)
        .serve(|| Ok(HelloWorldService));
}
