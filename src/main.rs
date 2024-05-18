use std::time::Duration;

use crate::chap10::{car_arrived, car_door_closed, car_door_opened, car_floor_button_pressed, lobby_call_button_pressed, Direction};

mod cahp8;
mod chap10;
mod chap12;

#[derive(Debug)]
struct Meters(i32);

#[derive(Debug)]
struct MetersSquared(i32);

trait Multiply {
    type Output;
    fn multiply(&self, other: &Self) -> Self::Output;
}

impl Multiply for Meters {
    type Output = MetersSquared;

    fn multiply(&self, other: &Self) -> Self::Output {
        MetersSquared(self.0 * other.0)
    }
}

fn main() {
    let puppy = Meters(32);
    println!("{:?}",puppy.multiply(&Meters(5)));
}
