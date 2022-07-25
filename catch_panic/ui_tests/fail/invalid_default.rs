use catch_panic::catch_panic;
use jni::JNIEnv;
use jni::sys::jobject;

#[catch_panic(default = "magic fairy dust")]
fn invalid_default(_env: JNIEnv) -> jobject {
    panic!();
}

fn main() {}
