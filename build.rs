#[cfg(windows)]
extern crate winres;

fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("resources/mouse.ico");
    res.compile().unwrap();
}
