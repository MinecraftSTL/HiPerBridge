#[cfg(all(target_os = "windows", target_env = "msvc"))]
fn main() {
    use winres::*;
    let mut res = WindowsResource::new();
    res.set_icon("assets/icon-slim.ico");
    res.set_icon_with_id("assets/icon-slim.ico", "ICON");
    res.set_icon_with_id("assets/icon-slim-gray.ico", "ICON_GRAY");
    // #[cfg(not(debug_assertions))]
    // res.set_manifest_file("assets/manifest.xml");
    res.compile().unwrap();
}

#[cfg(all(target_os = "windows", not(target_env = "msvc")))]
fn main() {
    update_self_info();
}

#[cfg(not(target_os = "windows"))]
fn main() {}
