use std::time::Duration;

use chap13::{do_things, StderrLogger, VerbosityFilter};

use crate::chap10::{car_arrived, car_door_closed, car_door_opened, car_floor_button_pressed, lobby_call_button_pressed, Direction};

mod cahp8;
mod chap10;
mod chap12;
mod chap13;

struct Point(i32, i32);

fn main() {
    {
        let p = Point(3, 4);
        println!("x: {}", p.0)
    }
    // error: cannot find value `p` in this scope
    println!("y: {}", p.1);
}
