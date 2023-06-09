use crate::*;




#[derive(Debug)]
pub enum Usefullness{
    Great,
    Ok,
    Bad,
}


#[derive(Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
pub struct ProjectCard {
    id: String,
    color: Color,
    name: Vec<Language>,
    cost: i32,
    requirements: Option<Vec<Requirement>>,
    tags: Vec<Tag>,
    effect_discription: Vec<Language>,
    card_resource: Option<CardResource>,
    action: Vec<Action>,
    effect: Vec<Effect>,
    on_card_action: Option<Vec<OnCardAction>>,
    vp: Option<VictoryPoint>,
    motivational_quote: Option<Vec<Language>>,
    bets_time_to_get: Option<(Usefullness, Usefullness, Usefullness)>,
    origin: Vec<Origin>
}

//setup implementation
impl ProjectCard {
    pub fn new(id: String, color: Color, name: Vec<Language>, cost: i32, effect_discription: Vec<Language>) -> ProjectCard {
        ProjectCard {
            id,
            color,
            name,
            cost,
            vp: None,
            tags: Vec::new(),
            requirements: None,
            card_resource: None,
            bets_time_to_get: None,
            on_card_action: None,
            motivational_quote: None,
            origin: Vec::new(),
            effect_discription,
            action: Vec::new(),
            effect: Vec::new(),
        }
    }
    pub fn add_on_card_action(mut self, on_card_action: OnCardAction)-> ProjectCard{
        let action = on_card_action;
        if let Some(ref mut on_card_action) = self.on_card_action {
            on_card_action.push(action);
        } else {
            self.on_card_action = Some(vec![action]);
        }
        self
    }
    pub fn add_on_card_actions(mut self, on_card_actions: Vec<OnCardAction>)-> ProjectCard{
        if let Some(ref mut on_card_action) = self.on_card_action {
            on_card_action.extend(on_card_actions);
        } else {
            self.on_card_action = Some(on_card_actions);
        }
        self
    }
    pub fn add_requirement(mut self, requirement: Requirement)-> ProjectCard {
        if let Some(mut requirements) = self.requirements {
            requirements.push(requirement);
            self.requirements = Some(requirements);
        }
        else {
            self.requirements = Some(vec![requirement])
        };
        self
    }
    pub fn set_vp(mut self, vp: VictoryPoint)-> Self{
        self.vp = Some(vp);
        self
    }
    pub fn add_tag(mut self, tag: Tag)-> Self{
        self.tags.push(tag);
        self
    }
    pub fn add_tags(mut self, tags: Vec<Tag>)-> Self{
        self.tags.extend(tags);
        self
    }
    pub fn set_card_resource(mut self, card_resource: CardResource)-> Self{
        self.card_resource = Some(card_resource);
        self
    }
    pub fn set_bets_time_to_get(mut self, bets_time_to_get: Option<(Usefullness, Usefullness, Usefullness)>)-> Self{
        self.bets_time_to_get = bets_time_to_get;
        self
    }
    pub fn add_motivational_quote(mut self, motivational_quote: Language)-> Self{
        let mot_quote = motivational_quote;
        if let Some(ref mut motivational_quote) =self.motivational_quote {
            motivational_quote.push(mot_quote);
        } else {
            self.motivational_quote = Some(vec![mot_quote]);
        }
        self
    }
    pub fn add_motivational_quotes(mut self, motivational_quotes: Vec<Language>)-> Self{
        let mot_quotes = motivational_quotes;
        if let Some(ref mut motivational_quote) =self.motivational_quote {
            motivational_quote.extend(mot_quotes);
        } else {
            self.motivational_quote = Some(mot_quotes);
        }
        self
    }
    pub fn add_origin(mut self, origin: Origin)-> Self{
        self.origin.push(origin);
        self
    }
}

impl PartialEq for ProjectCard {
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}


#[derive(Debug)]
pub enum Requirement {
    Tag(Vec<(Tag, usize, MinMax)>),
    Production(Resource),
    Tile(String),
    GlobalParameter(GlobalParameter, MinMax),
    TR(usize , MinMax),
    CardResource(CardResource, usize),
    Colony(MinMax),
    Chairman,
    RulingParty(TurmoilParty),
    TwoPartyLeaders,
    Custom(fn(&'static Game) -> Result<(), String>),
} 