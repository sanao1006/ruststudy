use crate::chap10::{car_arrived, car_door_closed, car_door_opened, car_floor_button_pressed, lobby_call_button_pressed, Direction};

mod cahp8;
mod chap10;


fn main() {
    let input = 'x';
    match input {
        'q'  => print!("Quitting"),
        'a' | 's' | 'w' | 'd' => println!("Moving around"),
        '0'..='9' => println!("Number Input"),
        key if key.is_lowercase() => println!("Lowercase: {key}").to_owned(),
        _ => println!("Something else"),
    }
}
