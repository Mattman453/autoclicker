#![windows_subsystem = "windows"]
use fltk::app::App;
use fltk::button::Button;
use fltk::enums::Color;
use fltk::image;
use fltk::prelude::{DisplayExt, GroupExt, WidgetBase, WidgetExt, WindowExt};
use fltk::text::{Cursor, TextBuffer, TextEditor};
use fltk::window::Window;
use fltk_theme::{color_themes, ColorTheme, SchemeType, WidgetScheme};
use global_hotkey::hotkey::{Code, HotKey};
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use mouse_rs::types::keys::Keys;
use mouse_rs::Mouse;
use rust_embed::Embed;
use std::num::ParseIntError;
use std::thread;
use std::thread::{sleep, JoinHandle};
use std::time::Duration;

#[derive(Embed)]
#[folder = "resources/"]
struct Asset;

fn set_global_hotkey(key: Code) -> GlobalHotKeyManager {
    let manager: GlobalHotKeyManager = GlobalHotKeyManager::new().unwrap();
    let hotkey: HotKey = HotKey::new(None, key);
    let _ = manager.register(hotkey);

    manager
}

fn set_scheme() {
    let widget_scheme: WidgetScheme = WidgetScheme::new(SchemeType::Fluent);
    widget_scheme.apply();

    let theme: ColorTheme = ColorTheme::new(color_themes::BLACK_THEME);
    theme.apply();
}

fn create_start_stop_btn() -> Button {
    let mut btn: Button = Button::new(110, 10, 80, 50, "Start (F6)");
    btn.set_color(Color::Background.lighter());

    btn.set_callback(move |b| {
        if b.label().contains("Start") {
            b.set_label("Stop (F6)");
        } else {
            b.set_label("Start (F6)");
        }
    });

    btn
}

fn create_txt_box() -> TextEditor {
    let mut buf: TextBuffer = TextBuffer::default();
    buf.set_text("100");

    let mut txt: TextEditor = TextEditor::new(10, 30, 80, 25, "Time (ms)");
    txt.set_buffer(buf);
    txt.set_color(Color::BackGround.lighter());
    txt.set_cursor_style(Cursor::Dim);

    txt
}

fn event_loop(win: &Window) {
    if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
        // println!("{:?}", event);
        if event.state == HotKeyState::Pressed {
            for i in 0..win.children() {
                if win.child(i).unwrap().label().contains("F6") {
                    win.child(i).unwrap().do_callback();
                }
            }
        }
    }
}

fn get_time(txt: &TextEditor) -> u64 {
    let string: String = txt.buffer().unwrap().text();
    let output: Result<u64, ParseIntError> = string.parse::<u64>();
    output.unwrap_or_else(|_| u64::MAX)
}

fn main() {
    let _manager = set_global_hotkey(Code::F6);

    let app: App = App::default();

    set_scheme();

    let mut win: Window = Window::default()
        .with_size(200, 70)
        .with_label("AutoClicker");
    let _btn = create_start_stop_btn();
    let txt: TextEditor = create_txt_box();

    win.end();
    win.show();

    let index_png = Asset::get("mouse.png").unwrap();
    let image = image::PngImage::from_data(&*index_png.data).unwrap();
    win.set_icon(Some(image));

    let mut thread: JoinHandle<_> = thread::spawn(move || {});

    while app.wait() {
        event_loop(&win);
        let time: u64 = get_time(&txt);
        for i in 0..win.children() {
            if win.child(i).unwrap().label().contains("Stop") {
                if thread.is_finished() {
                    thread.join().expect("Join failed");
                    thread = thread::spawn(move || {
                        let mouse = Mouse::new();
                        mouse.press(&Keys::LEFT).expect("Unable to press buttons");
                        mouse
                            .release(&Keys::LEFT)
                            .expect("Unable to release buttons");
                        sleep(Duration::from_millis(time));
                    });
                }
            }
        }
    }
}
