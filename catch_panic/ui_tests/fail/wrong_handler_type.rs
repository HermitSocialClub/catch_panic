use catch_panic::catch_panic;
use jni::JNIEnv;

pub fn trust_me(_env: JNIEnv, err: String) {
    // shhh :D
    drop(err);
}

#[catch_panic(handler = "trust_me")]
fn wrong_handler_type(_env: JNIEnv) {
    panic!();
}

fn main() {}
