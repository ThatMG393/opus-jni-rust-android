use std::cmp::{max, min};
use jni::JNIEnv;
use jni::objects::{JByteArray, JClass, JObject, JShortArray, JValue};
use jni::sys::{jboolean, jint, jlong, jshort};
use opus::{Application, Bitrate, Channels, Encoder};
use crate::encoder_container::EncoderContainer;
use crate::util::exception::{JavaException, JavaExceptions};
use crate::util::into_exception::ErrIntoException;
use crate::util::pointer::{get_pointer_from_field, JavaPointers};

#[no_mangle]
pub extern "system" fn Java_com_plasmoverse_opus_OpusEncoder_createNative(
    mut env: JNIEnv,
    _class: JClass,
    sample_rate: jint,
    stereo: jboolean,
    opus_mode: jint,
    mtu_size: jint
) -> jlong {
    match create_encoder(sample_rate, stereo, opus_mode, mtu_size) {
        Ok(pointer) => pointer,
        Err(exception) => {
            env.throw_new_exception(exception);
            0
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_plasmoverse_opus_OpusEncoder_resetNative(
    mut env: JNIEnv,
    encoder: JObject
) {
    match encoder_reset(&mut env, encoder) {
        Ok(pointer) => pointer,
        Err(exception) => {
            env.throw_new_exception(exception);
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_plasmoverse_opus_OpusEncoder_closeNative(
    mut env: JNIEnv,
    encoder: JObject
) {
    match encoder_close(&mut env, encoder) {
        Ok(pointer) => pointer,
        Err(exception) => {
            env.throw_new_exception(exception);
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_plasmoverse_opus_OpusEncoder_encodeNative<'local>(
    mut env: JNIEnv<'local>,
    encoder: JObject<'local>,
    samples: JShortArray<'local>
) -> JByteArray<'local> {
    match encoder_encode(&mut env, encoder, samples) {
        Ok(decoded) => decoded,
        Err(exception) => {
            env.throw_new_exception(exception);
            env.new_byte_array(0).expect("") // todo: ???
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_plasmoverse_opus_OpusEncoder_setBitrateNative(
    mut env: JNIEnv,
    encoder: JObject,
    bitrate: jint
) {
    match encoder_set_bitrate(&mut env, encoder, bitrate) {
        Ok(decoded) => decoded,
        Err(exception) => {
            env.throw_new_exception(exception);
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_plasmoverse_opus_OpusEncoder_getBitrateNative(
    mut env: JNIEnv,
    encoder: JObject,
) -> jint {
    match encoder_get_bitrate(&mut env, encoder) {
        Ok(decoded) => decoded,
        Err(exception) => {
            env.throw_new_exception(exception);
            0
        }
    }
}


fn create_encoder(
    sample_rate: jint,
    stereo: jboolean,
    opus_mode: jint,
    mtu_size: jint
) -> Result<jlong, JavaException> {
    let channels = match stereo {
        1u8 => Channels::Stereo,
        _ => Channels::Mono
    };

    let mode = match opus_mode {
        2049 => Application::Audio,
        2051 => Application::LowDelay,
        _ => Application::Voip
    };

    let encoder = Encoder::new(sample_rate as u32, channels, mode)
        .err_into_opus_exception("Failed to create encoder".into())?;

    let encoder_container = EncoderContainer {
        encoder,
        channels,
        mtu_size,
    };

    Ok(encoder_container.into_jlong_pointer())
}

unsafe fn get_encoder_container<'local>(
    env: &mut JNIEnv,
    encoder: &JObject
) -> Result<&'local mut EncoderContainer, JavaException> {
    let pointer = get_pointer_from_field(env, encoder, "pointer".into())
        .err_into_opus_exception("Failed to get a pointer from the java object".into())?;

    Ok(EncoderContainer::from_jlong_pointer(pointer))
}

unsafe fn encoder_reset(
    env: &mut JNIEnv,
    encoder: JObject
) -> Result<(), JavaException> {
    let container = get_encoder_container(env, &encoder)?;

    container.encoder.reset_state()
        .err_into_opus_exception("Failed to reset encoder state".into())?;

    Ok(())
}

unsafe fn encoder_close(
    env: &mut JNIEnv,
    encoder: JObject
) -> Result<(), JavaException> {
    let pointer = get_pointer_from_field(env, &encoder, "pointer".into())
        .err_into_opus_exception("Failed to get a pointer from the java object".into())?;

    let _container = Box::from_raw(pointer as *mut EncoderContainer);
    env.set_field(&encoder, "pointer", "J", JValue::from(0 as jlong))
        .err_into_opus_exception("Failed to reset pointer".into())?;

    Ok(())
}

unsafe fn encoder_encode<'local>(
    env: &mut JNIEnv<'local>,
    encoder: JObject<'local>,
    samples: JShortArray<'local>
) -> Result<JByteArray<'local>, JavaException> {
    let container = get_encoder_container(env, &encoder)?;

    let samples_length = env.get_array_length(&samples)
        .err_into_opus_exception("Failed to get samples array length".into())?
        as usize;

    let mut samples_vec = vec![0i16 as jshort; samples_length];

    env.get_short_array_region(samples, 0, &mut samples_vec)
        .err_into_opus_exception("Failed to copy samples to rust vec".into())?;

    let result = container.encoder.encode_vec(&samples_vec, container.mtu_size as usize)
        .err_into_opus_exception("Failed to encode audio".into())?;

    let encoded_java = env.byte_array_from_slice(&result)
        .err_into_opus_exception("Failed to create java byte array".into())?;

    Ok(encoded_java)
}

unsafe fn encoder_set_bitrate(
    env: &mut JNIEnv,
    encoder: JObject,
    bitrate: jint
) -> Result<(), JavaException> {
    let container = get_encoder_container(env, &encoder)?;

    let bitrate = match bitrate {
        -1000 => Bitrate::Auto,
        -1 => Bitrate::Max,
        _ => {
            Bitrate::Bits(min(max(bitrate, 500), 512_000))
        }
    };

    container.encoder.set_bitrate(bitrate)
        .err_into_opus_exception("Failed to get encoder bitrate".into())?;

    Ok(())
}

unsafe fn encoder_get_bitrate(
    env: &mut JNIEnv,
    encoder: JObject
) -> Result<jint, JavaException> {
    let container = get_encoder_container(env, &encoder)?;

    let bitrate = container.encoder.get_bitrate()
        .err_into_opus_exception("Failed to get encoder bitrate".into())?;

    let bitrate = match bitrate {
        Bitrate::Auto => -1000,
        Bitrate::Max => -1,
        Bitrate::Bits(bits) => bits
    };

    Ok(bitrate)
}
