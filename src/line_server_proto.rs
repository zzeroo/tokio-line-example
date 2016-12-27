use framed_transport::LineCodec;
use tokio_core::io::{Framed, Io};
use tokio_proto::pipeline::ServerProto;
use std::io;

pub struct LineServerProto;

impl<T: Io + 'static> ServerProto<T> for LineServerProto {
    type Request = String;
    type Response = String;
    type Error = io::Error;
    type Transport = Framed<T, LineCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LineCodec))
    }
}
