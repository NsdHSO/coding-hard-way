mod events;
mod ui;

use fltk::{app, prelude::*};

fn main() {
    let app = app::App::default();

    let mut win = ui::window::create_main_window();
    events::drag::attach_drag_events(&mut win);

    win.show();
    app.run().unwrap();
}
