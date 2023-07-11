use crate::*;


#[derive(Debug)]
pub struct Player {
    id: u8,
    name: String,
    //make hand able to hold copr and prelude too
    hand: Vec<ProjectCard>,
    // this one too
    played_cards: Vec<ProjectCard>,
    corporation: Vec<Corporation>,
    prelude: Vec<Prelude>,
    production: Production,
    resources: Resources,
    tags: Vec<(Tag, usize)>,
    vp: i16,
    terraform_rating: u8,
    actions: u8,
}


impl Player {
    pub fn new() -> Self {
        Player {
            id: 0,
            name: "".to_string(),
            hand: Vec::new(),
            played_cards: Vec::new(),
            corporation: Vec::new(),
            prelude: Vec::new(),
            production: Production::new(),
            resources: Resources::new(),
            tags: Vec::new(),
            vp: 0,
            terraform_rating: 20,
            actions: 2,
        }
    }
    pub fn hand(&self) -> &Vec<ProjectCard> {
        &self.hand
    }
    pub fn tags(&self) -> &Vec<(Tag, usize)> {
        &self.tags
    }/* 
    pub fn place_tile(&self, tile: OccupiedTileType) -> Result<(), String> {
        if !self.can_place_tile(&tile) {return Err("Not enough resources".to_string());}
        self.modify_resources(&tile.cost())?;
        Ok(())
    } */
}

#[derive(Debug)]
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
    pub fn modify_production(&mut self, resource: Resource) -> Result<(), String>{
        if !self.can_modify_production(&resource) {return Err("Not enough resources".to_string());}

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
    pub fn can_modify_production(&self, resource: &Resource) -> bool {
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

#[derive(Debug)]
pub struct Resources {
    money: u16,
    steel: u16,
    titanium: u16,
    plant: i16,
    energy: i16,
    heat: u16,
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

    pub fn modify_resources(&mut self, resource: Resource) -> Result<(), String>{
        self.can_modify_resources(&resource)?;

        match resource {
            Resource::Money(m) => self.money = (self.money as i32 + m as i32) as u16,
            Resource::Steel(s) => self.steel = (self.steel as i32 + s as i32) as u16,
            Resource::Titanium(t) => self.titanium = (self.titanium as i32 + t as i32) as u16,
            Resource::Plant(p) => self.plant += p,
            Resource::Energy(e) => self.energy += e,
            Resource::Heat(h) => self.heat = (self.heat as i32 + h as i32) as u16,
        }
        Ok(())
    }

    fn can_modify_resources(&self, resource: &Resource) -> Result<(), String> {
        let result = match resource {
            Resource::Money(n) => self.money as i32 + *n as i32 >= 0,
            Resource::Steel(n) => self.steel as i32 + *n as i32 >= 0,
            Resource::Titanium(n) => self.titanium as i32 + *n as i32 >= 0,
            Resource::Plant(n) => self.plant as i32 + *n as i32 >= 0,
            Resource::Energy(n) => self.energy as i32 + *n as i32 >= 0,
            Resource::Heat(n) => self.heat as i32 + *n as i32 >= 0,
        };
        if result {Ok(())} 
        else {
            let error_text = format!("Not enough {:?}", resource);
            Err(error_text)
        }
    }

    fn can_remove_plant_upto(&mut self, resource: &Resource) -> Result<bool, String> {
        match resource {
            Resource::Plant(_) => Ok(self.plant > 0),
            _ => Err("Not a plant resource".to_string()),
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