use behavior::TurnOnOffMachine;
use object_data::{describing, Car};

mod behavior;
mod object_data;

fn main() {
    let audi: Car = Car {
        name: String::from("car-audi"),
        model: String::from("Audi"),
        year: 2002,
    };
    println!("{}", audi.turn_on());
    println!("{}", audi.turn_off());
    describing(audi);
}
