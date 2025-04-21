use fltk::{app, enums::Event, prelude::*, window::Window};
use std::cell::Cell;
use std::rc::Rc;
pub fn attach_drag_events(win: &mut Window) {
    let is_dragging = Rc::new(Cell::new(false));
    let drag_offset_x = Rc::new(Cell::new(0));
    let drag_offset_y = Rc::new(Cell::new(0));

    let mut win_rc = win.clone();
    win.handle({
        let is_dragging = is_dragging.clone();
        let drag_offset_x = drag_offset_x.clone();
        let drag_offset_y = drag_offset_y.clone();

        move |w, ev| match ev {
            Event::Push => {
                let (x_root, y_root) = (app::event_x_root(), app::event_y_root());
                drag_offset_x.set(x_root - w.x());
                drag_offset_y.set(y_root - w.y());
                is_dragging.set(true);
                true
            }
            Event::Drag => {
                if is_dragging.get() {
                    let (x_root, y_root) = (app::event_x_root(), app::event_y_root());
                    let new_x = x_root - drag_offset_x.get();
                    let new_y = y_root - drag_offset_y.get();
                    w.set_pos(new_x, new_y);
                    true
                } else {
                    false
                }
            }
            Event::Released => {
                is_dragging.set(false);
                true
            }
            _ => false,
        }
    });

    win_rc.redraw(); // Optional: Ensures any position changes take effect immediately
}
