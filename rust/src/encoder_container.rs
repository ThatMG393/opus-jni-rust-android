use jni::sys::jlong;
use opus::{Channels, Encoder};
use crate::util::pointer::JavaPointers;

pub struct EncoderContainer {
    pub encoder: Encoder,
    pub channels: Channels,
    pub mtu_size: i32
}

impl JavaPointers<EncoderContainer> for EncoderContainer {
    fn into_jlong_pointer(self) -> jlong {
        Box::into_raw(Box::new(self)) as jlong
    }
}
