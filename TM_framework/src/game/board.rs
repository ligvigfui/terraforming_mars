



#[derive(Debug, Clone)]
pub enum MinMax {
    Min,
    Max,
}

#[derive(Debug, PartialEq, Clone)]
pub enum GlobalParameter {
    Temperature(i8),
    Oxygen(i8),
    Ocean(i8),
    Venus(i8),
}

#[derive(Debug, Clone)]
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