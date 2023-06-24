



#[derive(Debug)]
pub enum MinMax {
    Min,
    Max,
}

#[derive(Debug)]
pub enum GlobalParameter {
    Temperature(i8),
    Oxygen(u8),
    Ocean(u8),
    Venus(u8),
}

#[derive(Debug)]
pub struct GlobalParameters {
    temperature: i8,
    oxygen: u8,
    ocean: u8,
    venus: u8,
}

impl GlobalParameters {
    pub fn new() -> Self {
        GlobalParameters {
            temperature: -30,
            oxygen: 0,
            ocean: 0,
            venus: 0,
        }
    }
}