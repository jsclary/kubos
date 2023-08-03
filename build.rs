#[cfg(any(target_os = "windows", target_os = "linux"))]
extern crate winres;

fn main() {
    // TODO: A target_os of "linux" might not be sufficient. I think it's specifically an X Windows target
    // that it works with so this may break if we decide to support an SDL2 version.
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/kubos.ico"); // Replace this with the filename of your .ico file.
        res.compile().unwrap();
    }
}
