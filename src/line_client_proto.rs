use framed_transport::LineCodec;
use tokio_core::io::{Io, Framed};
use tokio_proto::pipeline::ClientProto;
use std::io;

pub struct LineClientProto;

impl<T: Io + 'static> ClientProto<T> for LineClientProto {
    type Request = String;
    type Response = String;
    type Error = io::Error;
    type Transport = Framed<T, LineCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LineCodec))
    }
}
