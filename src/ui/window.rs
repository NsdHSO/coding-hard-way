use fltk::enums::{Color, FrameType};
use fltk::{button::Button, frame::Frame, prelude::*, window::Window};

pub fn create_main_window() -> Window {
    let mut win = Window::default().with_size(400, 300);
    win.set_border(false);
    win.make_resizable(true);
    win.begin();
    let padding = 20;
    let mut padding_frame = Frame::new(
        padding / 2,
        padding / 2,
        win.w() - padding,
        win.h() - padding,
        "",
    );
    padding_frame.set_frame(FrameType::RoundedBox); // Use RoundBox for rounded corners
    win.set_frame(FrameType::RoundedBox); // Use RoundBox for rounded corners
    padding_frame.set_color(Color::White); // Set the background color

    let _button = Button::new(10, 10, 100, 30, "Click Me");

    win.end();
    win
}
