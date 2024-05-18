use std::time::Duration;

use crate::chap10::{car_arrived, car_door_closed, car_door_opened, car_floor_button_pressed, lobby_call_button_pressed, Direction};

mod cahp8;
mod chap10;
mod chap12;


#[derive(Debug)]
struct Race {
    name: String,
    laps: Vec<i32>,
}

impl Race {
    fn new(name: &str) -> Self {
        Self { name: String::from(name), laps: Vec::new() }
    }

    fn add_lap(&mut self, lap: i32) {
        self.laps.push(lap)
    }

    fn print_laps(&self) {
        println!("Recorded {} laps for {}:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("Lap {idx}: {lap} sec");
        }
    }

    fn finish(self) {
        let total: i32 = self.laps.iter().sum();
        println!("Race {} is finished, total lap time: {}", self.name, total);
    }
}

fn main() {
    let mut race = Race::new("Monaco Grand Prix");
    race.add_lap(32);
    race.add_lap(68);
    race.print_laps();
    race.add_lap(71);
    race.print_laps();
    race.finish();
}
