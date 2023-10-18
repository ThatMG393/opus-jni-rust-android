use jni::objects::{JByteArray, JClass, JObject, JShortArray, JValue};
use jni::{JNIEnv};
use jni::sys::{jboolean, jint, jlong, jsize};
use opus::{Channels, Decoder};
use crate::decoder_container::DecoderContainer;
use crate::util::exception::{JavaException, JavaExceptions};
use crate::util::into_exception::ErrIntoException;
use crate::util::pointer::{get_pointer_from_field, JavaPointers};

#[no_mangle]
pub extern "system" fn Java_com_plasmoverse_opus_OpusDecoder_createNative(
    mut env: JNIEnv,
    _class: JClass,
    sample_rate: jint,
    stereo: jboolean,
    frame_size: jint
) -> jlong {
    match create_decoder(sample_rate, stereo, frame_size) {
        Ok(pointer) => pointer,
        Err(exception) => {
            env.throw_new_exception(exception);
            0
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_plasmoverse_opus_OpusDecoder_resetNative(
    mut env: JNIEnv,
    decoder: JObject
) {
    match decoder_reset(&mut env, decoder) {
        Ok(pointer) => pointer,
        Err(exception) => {
            env.throw_new_exception(exception);
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_plasmoverse_opus_OpusDecoder_closeNative(
    mut env: JNIEnv,
    decoder: JObject
) {
    match decoder_close(&mut env, decoder) {
        Ok(pointer) => pointer,
        Err(exception) => {
            env.throw_new_exception(exception);
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_plasmoverse_opus_OpusDecoder_decodeNative<'local>(
    mut env: JNIEnv<'local>,
    decoder: JObject<'local>,
    encoded: JByteArray<'local>
) -> JShortArray<'local> {
    match decoder_decode(&mut env, decoder, encoded) {
        Ok(decoded) => decoded,
        Err(exception) => {
            env.throw_new_exception(exception);
            env.new_short_array(0).expect("") // todo: ???
        }
    }
}


fn create_decoder(
    sample_rate: jint,
    stereo: jboolean,
    frame_size: jint
) -> Result<jlong, JavaException> {
    let channels = match stereo {
        1u8 => Channels::Stereo,
        _ => Channels::Mono
    };

    let decoder = Decoder::new(sample_rate as u32, channels)
        .err_into_opus_exception("Failed to create decoder".into())?;

    let decoder_container = DecoderContainer {
        decoder,
        channels,
        frame_size,
    };

    Ok(decoder_container.into_jlong_pointer())
}

unsafe fn get_decoder_container<'local>(
    env: &mut JNIEnv,
    decoder: &JObject
) -> Result<&'local mut DecoderContainer, JavaException> {
    let pointer = get_pointer_from_field(env, decoder, "pointer".into())
        .err_into_opus_exception("Failed to get a pointer from the java object".into())?;

    Ok(DecoderContainer::from_jlong_pointer(pointer))
}

unsafe fn decoder_reset(
    env: &mut JNIEnv,
    decoder: JObject
) -> Result<(), JavaException> {
    let container = get_decoder_container(env, &decoder)?;

    container.decoder.reset_state()
        .err_into_opus_exception("Failed to reset decoder state".into())?;

    Ok(())
}

unsafe fn decoder_close(
    env: &mut JNIEnv,
    decoder: JObject
) -> Result<(), JavaException> {
    let pointer = get_pointer_from_field(env, &decoder, "pointer".into())
        .err_into_opus_exception("Failed to get a pointer from the java object".into())?;

    let _container = Box::from_raw(pointer as *mut DecoderContainer);
    env.set_field(&decoder, "pointer", "J", JValue::from(0 as jlong))
        .err_into_opus_exception("Failed set reset pointer".into())?;

    Ok(())
}

unsafe fn decoder_decode<'local>(
    env: &mut JNIEnv<'local>,
    decoder: JObject<'local>,
    encoded: JByteArray<'local>
) -> Result<JShortArray<'local>, JavaException> {
    let container = get_decoder_container(env, &decoder)?;

    let encoded = match encoded.is_null() {
        true => vec![0u8; 0],
        false => env.convert_byte_array(encoded)
            .err_into_opus_exception("Failed to convert byte array to rust vec".into())?
    };

    let mut decoded = vec![0i16; container.frame_size as usize * container.channels as usize];

    let result = container.decoder.decode(&encoded, &mut decoded, false)
        .err_into_opus_exception("Failed to decode audio".into())?;

    let result_length = result * container.channels as usize;
    decoded.truncate(result_length);

    let decoded_java = env.new_short_array(result_length as jsize)
        .err_into_opus_exception("Failed to create java short array".into())?;

    env.set_short_array_region(&decoded_java, 0, &decoded)
        .err_into_opus_exception("Failed to copy short vec into java short array".into())?;

    Ok(decoded_java)
}
