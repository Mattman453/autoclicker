#[cfg(windows)]
extern crate winres;

fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("resources/mouse.ico");
    match res.compile() {
        Ok(_) => {}
        Err(_) => {
            println!("File \"resources/mouse.ico\" not found. No icon set for app.");
        }
    }
}
