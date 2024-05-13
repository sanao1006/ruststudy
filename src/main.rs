mod cahp8;

enum CarryableConcreteItem {
    Left,
    Right
}

type Item = CarryableConcreteItem;

use std::cell::RefCell;
use std::sync::{Arc, RwLock};

type PlayerInventory = RwLock<Vec<Arc<RefCell<Item>>>>;

fn main() {

}
