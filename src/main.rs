#[cfg(not(target_os = "android"))]
fn main() {
    // This bit of indirection is necessary for Android.
    kubos::entry_point()
}

// Android doesn't use the bin output but the compiler complains otherwise.
#[cfg(target_os = "android")]
fn main() {}
