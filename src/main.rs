use crate::chap10::{car_arrived, car_door_closed, car_door_opened, car_floor_button_pressed, lobby_call_button_pressed, Direction};

mod cahp8;
mod chap10;

struct Foo {
    x: (u32, u32),
    y:u32,
}

fn main() {
    let foo = Foo { x: (2, 2), y: 2};
    match foo {
        Foo { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
        Foo {y: 2, x: i} => println!("y = 2, x = {i:?}"),
        Foo { y, .. } => println!("y = {y}, other fields were ignored")
    }
}
