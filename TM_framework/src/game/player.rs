use crate::*;

pub static mut PLAYER_ID: u8 = 255;

#[derive(Debug, Clone)]
pub struct Player {
    pub id: u8,
    pub name: String,
    pub(crate) research: Vec<ProjectCard>,
    //make hand able to hold corp and prelude too
    pub(crate) hand: Vec<ProjectCard>,
    // this one too
    pub(crate) played_cards: Vec<ProjectCard>,
    pub(crate) corporation: Vec<Corporation>,
    pub(crate) prelude: Vec<Prelude>,
    pub(crate) production: Production,
    pub(crate) resources: Resources,
    pub(crate) tags: Vec<(Tag, usize)>,
    pub(crate) vp: i16,
    pub(crate) terraform_rating: u8,
    pub(crate) actions_remaining: u8,
    pub(crate) characteristics: PlayerCharacteristics,
}


impl Player {
    pub fn new() -> Self {
        Player {
            id: unsafe { PLAYER_ID += 1; PLAYER_ID },
            name: format!("Player {}", unsafe { PLAYER_ID }),
            research: Vec::new(),
            hand: Vec::new(),
            played_cards: Vec::new(),
            corporation: Vec::new(),
            prelude: Vec::new(),
            production: Production::new(),
            resources: Resources::new(),
            tags: Vec::new(),
            vp: 0,
            terraform_rating: 20,
            actions_remaining: 2,
            characteristics: PlayerCharacteristics::new(),
        }
    }
    /* 
    pub fn place_tile(&self, tile: OccupiedTileType) -> Result<(), String> {
        if !self.can_place_tile(&tile) {return Err("Not enough resources".to_string());}
        self.modify_resources(&tile.cost())?;
        Ok(())
    } */
}

#[derive(Debug, Clone)]
pub struct PlayerCharacteristics {
    pub(crate) card_cost: u8,
    pub(crate) steel_value: u8,
    pub(crate) titanium_value: u8,
    pub(crate) plants_to_greenery: u8,
}

impl PlayerCharacteristics {
    pub(crate) fn new() -> Self {
        PlayerCharacteristics {
            card_cost: 3,
            steel_value: 2,
            titanium_value: 3,
            plants_to_greenery: 8,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Production {
    money: i16,
    steel: i16,
    titanium: i16,
    plant: i16,
    energy: i16,
    heat: i16,
}

impl Production {
    pub fn new() -> Self {
        Production {
            money: 0,
            steel: 0,
            titanium: 0,
            plant: 0,
            energy: 0,
            heat: 0,
        }
    }
    pub fn modify(&mut self, resource: Resource) -> Result<(), String>{
        if !self.can_modify(&resource) {return Err("Not enough production".to_string());}

        match resource {
            Resource::Money(m) => self.money += m,
            Resource::Steel(s) => self.steel += s,
            Resource::Titanium(t) => self.titanium += t,
            Resource::Plant(p) => self.plant += p,
            Resource::Energy(e) => self.energy += e,
            Resource::Heat(h) => self.heat += h,
        }
        Ok(())
    }
    pub fn can_modify(&self, resource: &Resource) -> bool {
        match resource {
            Resource::Money(n) => self.money + n >= -5,
            Resource::Steel(n) => self.steel + n >= 0,
            Resource::Titanium(n) => self.titanium + n >= 0,
            Resource::Plant(n) => self.plant + n >= 0,
            Resource::Energy(n) => self.energy + n >= 0,
            Resource::Heat(n) => self.heat + n >= 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Resources {
    pub(crate) money: u16,
    pub(crate) steel: u16,
    pub(crate) titanium: u16,
    pub(crate) plant: u16,
    pub(crate) energy: u16,
    pub(crate) heat: u16,
}

impl Resources {
    pub fn new() -> Self {
        Resources {
            money: 0,
            steel: 0,
            titanium: 0,
            plant: 0,
            energy: 0,
            heat: 0,
        }
    }

    pub fn modify(&mut self, resource: Resource) -> Result<(), String>{
        if !self.can_modify(&resource) {return Err("Not enough resources".to_string());}

        match resource {
            Resource::Money(m) => self.money = (self.money as i32 + m as i32) as u16,
            Resource::Steel(s) => self.steel = (self.steel as i32 + s as i32) as u16,
            Resource::Titanium(t) => self.titanium = (self.titanium as i32 + t as i32) as u16,
            Resource::Plant(p) => self.plant = (self.plant as i32 + p as i32) as u16,
            Resource::Energy(e) => self.energy = (self.energy as i32 + e as i32) as u16,
            Resource::Heat(h) => self.heat = (self.heat as i32 + h as i32) as u16,
        }
        Ok(())
    }

    pub fn can_modify(&self, resource: &Resource) -> bool {
        match resource {
            Resource::Money(n) => self.money as i32 + *n as i32 >= 0,
            Resource::Steel(n) => self.steel as i32 + *n as i32 >= 0,
            Resource::Titanium(n) => self.titanium as i32 + *n as i32 >= 0,
            Resource::Plant(n) => self.plant as i32 + *n as i32 >= 0,
            Resource::Energy(n) => self.energy as i32 + *n as i32 >= 0,
            Resource::Heat(n) => self.heat as i32 + *n as i32 >= 0,
        }
    }

    pub fn remove_upto(&mut self, resource: Resource) -> Result<(), String> {
        match resource {
            Resource::Money(n) => {
                if self.money as i32 <= n as i32 { self.money = 0; }
                else { self.money -= n as u16; }
                Ok(())
            },
            Resource::Steel(n) => {
                if self.steel as i32 <= n as i32 { self.steel = 0; }
                else { self.steel -= n as u16; }
                Ok(())
            },
            Resource::Titanium(n) => {
                if self.titanium as i32 <= n as i32 { self.titanium = 0; }
                else { self.titanium -= n as u16; }
                Ok(())
            },
            Resource::Plant(n) => {
                if self.plant as i32 <= n as i32 { self.plant = 0; }
                else { self.plant -= n as u16; }
                Ok(())
            },
            Resource::Energy(n) => {
                if self.energy as i32 <= n as i32 { self.energy = 0; }
                else { self.energy -= n as u16; }
                Ok(())
            },
            Resource::Heat(n) => {
                if self.heat as i32 <= n as i32 { self.heat = 0; }
                else { self.heat -= n as u16; }
                Ok(())
            },
        }
    }

    pub fn can_remove_any(&mut self, resource: &Resource) -> bool {
        match resource {
            Resource::Money(_) => self.money > 0,
            Resource::Steel(_) => self.steel > 0,
            Resource::Titanium(_) => self.titanium > 0,
            Resource::Plant(_) => self.plant > 0,
            Resource::Energy(_) => self.energy > 0,
            Resource::Heat(_) => self.heat > 0,
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub enum Resource {
    Money(i16),
    Steel(i16),
    Titanium(i16),
    Plant(i16),
    Energy(i16),
    Heat(i16),
}