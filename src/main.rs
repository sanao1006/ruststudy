use std::time::Duration;

use chap13::{do_things, StderrLogger, VerbosityFilter};

use crate::chap10::{car_arrived, car_door_closed, car_door_opened, car_floor_button_pressed, lobby_call_button_pressed, Direction};

mod cahp8;
mod chap10;
mod chap12;
mod chap13;


fn say_hello(name: String) {
    println!("Hello {name}")
}

fn main() {
    let name = String::from("Alice");
    say_hello(name.clone());
    say_hello(name);
    // say_hello(name);
}