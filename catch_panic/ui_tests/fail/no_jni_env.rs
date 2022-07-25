use catch_panic::catch_panic;

#[catch_panic]
fn no_jni_env() {
    panic!();
}

fn main() {}
