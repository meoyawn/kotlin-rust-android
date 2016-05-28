extern crate jni_sys;
extern crate cesu8;
use jni_sys::*;

#[derive(Debug)]
struct ExceptionThrown;

#[derive(Copy, Clone)]
struct SafeJNI(*mut jni_sys::JNIEnv);

impl SafeJNI {
    fn string(&self, s: &str) -> Result<jni_sys::jstring, ExceptionThrown> {
        let s = cvt(s);
        let s = unsafe { ((**self.0).NewStringUTF)(self.0, s.as_ptr() as *const _) };
        if s.is_null() {
            return Err(ExceptionThrown);
        }
        Ok(s)
    }
}

fn cvt(s: &str) -> Vec<u8> {
    let mut s = cesu8::to_java_cesu8(s).to_vec();
    s.push(0);
    s
}

#[no_mangle]
pub extern "C" fn Java_adeln_kotlinrustandroid_MainActivityKt_test(env: *mut JNIEnv,
                                                                   class: jclass)
                                                                   -> jstring {
    SafeJNI(env).string("this is rust mate").unwrap()
}
