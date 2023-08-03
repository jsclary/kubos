#[cfg(not(any(target_os = "android", target_os = "ios", target_arch = "wasm32")))]
fn main() {
    // This bit of indirection is necessary for Android.
    libkubos::entry_point()
}

// Android doesn't use the bin output but the compiler complains otherwise.
#[cfg(any(target_os = "android", target_os = "ios", target_arch = "wasm32"))]
fn main() {}
