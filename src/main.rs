use std::time::Duration;

use crate::chap10::{car_arrived, car_door_closed, car_door_opened, car_floor_button_pressed, lobby_call_button_pressed, Direction};

mod cahp8;
mod chap10;

fn sleep_for(secs: f32) {
    if let Ok(dur) = Duration::try_from_secs_f32(secs)  {
        std::thread::sleep(dur);
        println!("slept for {:?}", dur)
        
    }
}

fn main() {
    sleep_for(-10.0);
    sleep_for(0.8)
}
