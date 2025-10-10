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

fn dangling () {
    let t;
    {
        let x = 5;
        t = &x;
    }
    println!("{}", t);
}

#[test]
fn check_dangling() {
    dangling()
}



#[test]
fn borrowing() {
    let t = 3;
    let mut y = &t;
    println!("{}", y);

    y = &30;
    println!("{}", y);
}



#[test]
fn integer_check() {
    let t = 3;
    assert_eq!(check_about_integer(t), 8)
}
fn check_copy_integer(mut a: i32) ->i32 {
    a = 3;
    a
}

#[test]
fn copy_integer() {
    let mut a = 50;
    check_copy_integer(a);
    assert_eq!(a, 50);
}
fn check_about_integer(a: i32) -> i32 {
    let mut t = a;
    t = 5;
    println!("{}, {}", a, t);
    a + t
}

fn check_string_literal(s1: &mut String) {
    let mut s = "Hello";
    println!("{}", s);

    s = "World";
    println!("{}", s);

    s1.push_str(", World!");
    println!("{}", s1);
}

#[test]
fn check_string() {
    let mut s1 = String::from("Hello");
    check_string_literal(&mut s1);
    s1.push_str(", World!");
    println!("{}", s1);
}

fn add_two_number<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T> + Copy,
{
    a + b
}

#[test]
fn check_add_number() {
    assert_eq!(add_two_number(1.3, 2.2), 3.5);
}
