
#[derive(Debug)]
/// An event in the elevator system that the controller must react to.
pub enum Event {
    Arrived(Floor),
    Opend,
    Closed,
    ButtonPressed(Button)
}

#[derive(Debug)]
enum Button {
    LobbyCall(Floor, Direction),
    CarFloor(Floor),
}

type Floor = i32;

/// A direction of travel.
#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
}

/// The car has arrived on the given floor.
pub fn car_arrived(floor: i32) -> Event {
    return Event::Arrived(floor);
}

/// The car doors have opened.
pub fn car_door_opened() -> Event {
    return Event::Opend
}

/// The car doors have closed.
pub fn car_door_closed() -> Event {
    return Event::Closed
}

/// A directional button was pressed in an elevator lobby on the given floor.
pub fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    return Event::ButtonPressed(Button::LobbyCall(floor, dir))
}

/// A floor button was pressed in the elevator car.
pub fn car_floor_button_pressed(floor: i32) -> Event {
    return Event::ButtonPressed(Button::CarFloor(floor));
}
