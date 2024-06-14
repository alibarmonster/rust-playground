use crate::behavior::TurnOnOffMachine;

pub struct Car {
    pub name: String,
    pub model: String,
    pub year: i32,
}

impl TurnOnOffMachine for Car {
    fn turn_on(&self) -> String {
        format!(
            "Car name: {}, with model {} and year {} just turn on the machine",
            self.name, self.model, self.year
        )
    }

    fn turn_off(&self) -> String {
        format!("Car name: {} just turned off", self.name)
    }

    fn describe_on_off_machine(&self) -> String {
        format!("Car name: {}, should not on_off machine every 5 minutes, that will caused broken machine", self.name)
    }
}

pub fn describing<T: TurnOnOffMachine>(item: T) {
    println!("what car broke? {}", item.describe_on_off_machine());
}

// pub struct MotorCycle {
//     pub name: String,
//     pub model: String,
//     pub year: i32,
// }
//
// impl TurnOnOffMachine for MotorCycle {
//     fn turn_on(&self) -> String {
//         format!(
//             "Motorcycle name: {}, with model {}, and year {} just turn on the machine",
//             self.name, self.model, self.year
//         )
//     }
//
//     fn turn_off(&self) -> String {
//         format!("Motorcycle name: {} just turned off ", self.name)
//     }
// }
