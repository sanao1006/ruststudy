use std::time::Duration;

use chap13::{do_things, StderrLogger, VerbosityFilter};

use crate::chap10::{car_arrived, car_door_closed, car_door_opened, car_floor_button_pressed, lobby_call_button_pressed, Direction};

mod cahp8;
mod chap10;
mod chap12;
mod chap13;


fn main() {
    let l = VerbosityFilter { max_verbosity: 3, inner: StderrLogger };
    do_things(&l);
}
