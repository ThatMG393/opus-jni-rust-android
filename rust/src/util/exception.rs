use jni::JNIEnv;

pub struct JavaException {
    class: String,
    message: String
}

impl JavaException {

    pub fn new_illegal_argument(message: String) -> JavaException {
        JavaException {
            class: "java/lang/IllegalArgumentException".into(),
            message
        }
    }

    pub fn new_illegal_state(message: String) -> JavaException {
        JavaException {
            class: "java/lang/IllegalStateException".into(),
            message
        }
    }

    pub fn new_opus(message: String) -> JavaException {
        JavaException {
            class: "com/plasmoverse/opus/OpusException".into(),
            message
        }
    }
}

pub trait JavaExceptions {

    fn throw_new_exception(&mut self, exception: JavaException);
}

impl<'local> JavaExceptions for JNIEnv<'local> {

    fn throw_new_exception(&mut self, exception: JavaException) {
        println!("{} {}", exception.class, exception.message);
        let _ = self.throw_new(exception.class, exception.message);
    }
}
