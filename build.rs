extern crate winres;

fn main() {
    if cfg!(target_env = "msvc") && cfg!(target_env = "gnu") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/kubos.ico"); // Replace this with the filename of your .ico file.
        res.compile().unwrap();
    }
}
