use catch_panic::catch_panic;
use jni::JNIEnv;

#[catch_panic(handler = "accepting panic donations --> |_|")]
fn invalid_handler(_env: JNIEnv) {
    panic!();
}

fn main() {}
