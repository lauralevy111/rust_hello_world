#![allow(dead_code)]

enum Status {
    Rich,
    Poor,

}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // Explicitly `use` each name so they are available without
    // manual scoping.
    use crate::Status::{Poor, Rich};

}
