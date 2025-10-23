mod events;
mod ui;

use fltk::{app, prelude::*};
use std::fmt::{Debug, Formatter};

fn main() {
    let app = app::App::default();

    let mut win = ui::window::create_main_window();
    events::drag::attach_drag_events(&mut win);

    win.show();
    app.run().unwrap();
}

#[cfg(test)]
mod Trait {
    pub trait Sound {
        fn play(&self) {
            println!("I am playing!");
        }
    }

    struct Bird;
    struct Dog;
    struct Cat;
    struct Fish;

    impl Sound for Bird {
        fn play(&self) {
            println!("Chirp chirp!")
        }
    }

    const chicken: Bird = Bird;

    fn play_sound<T: Sound>(sound: &T) {
        println!("Playing sound");
        sound.play();
    }

    #[test]
    fn check_sound() {
        play_sound(&chicken);
    }
}

#[cfg(test)]
mod HasMapTest {
    use std::collections::HashMap;

    #[test]
    fn creating_hash() {
        let names = Vec::from(["John", "Jane", "Josh", "Jess", "Jake"]);
        let grades = [1, 2, 34, 5, 5];

        let mut student_grades: HashMap<&str, i32> =
            names.into_iter().zip(grades.into_iter()).collect();

        let mut borr_grades1 = &mut student_grades;
        use_mut_borrow(&mut borr_grades1);
        println!("{:?}", student_grades);
        student_grades.insert("John", 100);
        println!("{:?}", student_grades);
    }

    fn use_borrow(t: &HashMap<&str, i32>) {
        println!("{:?}", t);
    }

    fn use_mut_borrow(t: &mut HashMap<&str, i32>) {
        t.insert("Jane", 1000);
        println!("{:?}", t);
    }
}

#[cfg(test)]
mod StructsTest {
    use std::fmt::{Display, Formatter};

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Display for Rectangle {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", self.width, self.height)
        }
    }

    #[test]
    fn struct_rectangle() {
        let r1 = Rectangle {
            width: 2,
            height: 54,
        };
        let r3 = Rectangle { width: 32, ..r1 };
        let mut r4 = Rectangle { width: 32, ..r1 };

        print_height_immutable(&r1);
        print_height_mutable_and_modified(&mut r4);
        assert_eq!(r1.width, 2);
        assert_eq!(r4.height, 20);
        assert_eq!(r3.height, 54);
        println!("{}", r4);
    }

    fn print_height_immutable(rect: &Rectangle) {
        println!("{}", rect.height)
    }
    fn print_height_mutable_and_modified(rect: &mut Rectangle) {
        println!("{}", rect.height);
        rect.height = 20;
        println!("---- in function {}", rect.height);
    }
}

#[cfg(test)]
mod basic_test {
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
    fn check_copy_integer(mut a: i32) -> i32 {
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
}

pub trait SoundCar: Debug {
    fn play(&self) {
        println!("I am playing!");
    }
}

pub trait Light {
    fn on(&self) {
        println!("Light is on!");
    }
    fn off(&self) {
        println!("Light is off!");
    }
}

struct BMW {
    chair: bool,
}

impl Debug for BMW {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BMW").field("chair", &self.chair).finish()
    }
}

impl SoundCar for BMW {
    fn play(&self) {
        println!("Chirp chirp!")
    }
}

impl Light for BMW {
    fn on(&self) {
        println!("BMW Light is on!")
    }
    fn off(&self) {
        println!("BMW Light is off!")
    }
}
#[cfg(test)]
mod check_implementation_trait {
    use super::*;
    #[test]
    fn check_implementation_trait_for_car() {
        let m5 = BMW { chair: true };

        play_sound(&m5);
        turn_on(&m5);
        let checkSoundM5 = check_return_type(&m5);
        println!("{:?}", checkSoundM5);
    }

    fn check_return_type(sound: &impl SoundCar) -> impl SoundCar {
        BMW { chair: true }
    }

    fn play_sound(sound: &impl SoundCar) {
        println!("Playing sound");
        sound.play();
    }
    fn turn_on(sound: &(impl SoundCar + Light)) {
        println!("Playing sound");
        sound.play();
    }
}
