use jni::sys::jlong;
use opus::{Channels, Decoder};
use crate::util::pointer::JavaPointers;

pub struct DecoderContainer {
    pub decoder: Decoder,
    pub channels: Channels,
    pub buffer_size: i32
}

impl JavaPointers<DecoderContainer> for DecoderContainer {
    fn into_jlong_pointer(self) -> jlong {
        Box::into_raw(Box::new(self)) as jlong
    }
}
