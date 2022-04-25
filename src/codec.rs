use actix_codec::{Decoder, Encoder};
use actix_web::web;
use bytes::{Buf, Bytes, BytesMut};
use std::io;

pub struct WebBytesCodec;

impl Encoder<Bytes> for WebBytesCodec {
    type Error = io::Error;

    #[inline]
    fn encode(&mut self, item: Bytes, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.extend_from_slice(item.chunk());
        Ok(())
    }
}

impl Decoder for WebBytesCodec {
    type Item = web::Bytes;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            Ok(None)
        } else {
            Ok(Some(web::Bytes::from(src.split())))
        }
    }
}
