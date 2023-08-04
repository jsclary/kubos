#[cfg(any(target_env = "msvc", target_env="gnu"))]
extern crate winres;

fn main() {

    // Using cfg alone doesn't work because build.rs is compiled
    // for the host OS regardless of the crate build target.
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
 
        #[cfg(any(target_env = "msvc", target_env="gnu"))]
        winres::WindowsResource::new()
            .set_icon("assets/kubos.ico")
            .set("InternalName", "kubos.exe")
            .compile()
            .unwrap();

        // If we try to cross-builld an exe from mac or a musl 
        // linux distro, warn that the icon won't be built.
        #[cfg(not(any(target_env = "msvc", target_env="gnu")))]
        warn!("The application icon for windows executables can only be added on windows and gnu linux.");
    }
}
