//disable warn dead code
#![allow(dead_code)]


pub mod cards{
    

    use std::{vec, fmt::{Debug}};

    #[derive(Debug)]
    enum CardType {
        Card,
        Corporation,
        Prelude,
        Event,
    }

    #[derive(Debug)]
    enum Phase {
        Draft,
        Research,
        Action,
        Production,
        End,
        WorldGovernmentTerraforming,
        ColonyProduction,
        Turmoil,
    }

    #[derive(Debug)]
    pub enum Origin{
        Base,
        CorporateEra,
        Prelude,
        VenusNext,
        Colonies,
        Turmoil,
        Promo,
        Custom(String),
    }

    #[derive(Debug)]
    pub struct Card {
        id: String,
        color: Color,
        name: Vec<Language>,
        cost: i32,
        vp: VictoryPoint,
        tags: Vec<Tag>,
        requirements: Option<Requirement>,
        card_resource: Option<CardResource>,
        bets_time_to_get: Option<(Usefullness, Usefullness, Usefullness)>,
        effects: Vec<Language>,
        on_card_action: Option<Vec<OnCardAction>>,
        motivational_quote: Option<Vec<Language>>,
        origin: Vec<Origin>
    }

    //setup implementation
    impl Card {
        pub fn new(id: String, color: Color, name: Vec<Language>, cost: i32, effects: Vec<Language>) -> Card {
            Card {
                id,
                color,
                name,
                cost,
                vp: VictoryPoint::None,
                tags: Vec::new(),
                requirements: None,
                card_resource: None,
                bets_time_to_get: None,
                effects,
                on_card_action: None,
                motivational_quote: None,
                origin: Vec::new(),
            }
        }
        pub fn add_on_card_action(mut self, on_card_action: OnCardAction)-> Card{
            let action = on_card_action;
            if let Some(ref mut on_card_action) = self.on_card_action {
                on_card_action.push(action);
            } else {
                self.on_card_action = Some(vec![action]);
            }
            self
        }
        pub fn add_on_card_actions(mut self, on_card_actions: Vec<OnCardAction>)-> Card{
            if let Some(ref mut on_card_action) = self.on_card_action {
                on_card_action.extend(on_card_actions);
            } else {
                self.on_card_action = Some(on_card_actions);
            }
            self
        }
        pub fn set_requironment(mut self, requirement: Requirement)-> Card{
            self.requirements = Some(requirement);
            self
        }
        pub fn set_vp(mut self, vp: VictoryPoint)-> Self{
            self.vp = vp;
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
        pub fn set_card_resource(mut self, card_resource: CardResource)->Self{
            self.card_resource = Some(card_resource);
            self
        }
        pub fn set_bets_time_to_get(mut self, bets_time_to_get: Option<(Usefullness, Usefullness, Usefullness)>)->Self{
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
        pub fn add_origin(mut self, origin: Origin)->Self{
            self.origin.push(origin);
            self
        }
    }
    
    impl PartialEq for Card {
        fn eq(&self, other: &Self) -> bool {
            format!("{:?}", self) == format!("{:?}", other)
        }
    }

    //extern crate implementation
    impl Card {
        pub fn tags(&self) -> &Vec<Tag> {
            &self.tags
        }
    }
    
    #[derive(Debug)]
    pub enum Usefullness{
        Great,
        Ok,
        Bad,
    }





    #[derive(Debug)]
    ///! still not good need to change milestone, award, phase, history, include turmoil, colonies
    pub struct Game {
        players: Vec<Player>,
        deck: Vec<Card>,
        discard: Vec<Card>,
        current_player: usize,
        generation: usize,
        map: String,
        temperature: i32,
        oxygen: usize,
        ocean: usize,
        venus: usize,
        milestones: Vec<Card>,
        awards: Vec<Card>,
        phase: String,
        history: String,
    }

    impl Game {
        pub fn currentPlayer(&self) -> &Player {
            &self.players[self.current_player as usize]
        }
    }

    #[derive(Debug)]
    ///! still not good need to change milestone, award, action
    pub struct Player {
        id: usize,
        name: String,
        hand: Vec<Card>,
        played_cards: Vec<Card>,
        production: Vec<Resource>,
        resources: Vec<Resource>,
        tags: Vec<(Tag, usize)>,
        vp: i32,
        terraform_rating: usize,
        corporation: Vec<Corporation>,
        prelude: Vec<Prelude>,
        milestones: Vec<Card>,
        awards: Vec<Card>,
        actions: usize,
    }

    impl Player {
        pub fn hand(&self) -> &Vec<Card> {
            &self.hand
        }
        pub fn tags(&self) -> &Vec<(Tag, usize)> {
            &self.tags
        }
    }

    #[derive(Debug)]
    struct Corporation {
        name: Vec<Language>,
        effect: Vec<Language>,
        description: Vec<Language>,
        tags: Vec<Tag>,
    }

    #[derive(Debug)]
    struct Prelude {
        name: Vec<Language>,
        description: Vec<Language>,
        effect: Vec<Language>,
    }

    pub struct Effect {
        criteria: Option<OnCardAction>,
        reward: OnCardAction,
    }

    #[derive(Debug)]
    pub enum OnCardAction {
        // move card from research to hand
        BuyCard(usize),
        // draw random card from deck
        DrawCard(usize),
        // move card from research or hand to discard
        Discard(usize),
        ModifyResources(Resource),
        ModifyProduction(Resource),
        MustRemoveFromAnyPlayersResources(Resource), 
        MustRemoveFromAnyPlayersProduction(Resource),
        RemoveFromAnyPlayersResources(Resource),
        PlaceTile,
        RemoveTile,
        ModifyTerraformRating(i32),
        ModifyGlobalParameter(GlobalParameter, i32),
        ModifyCardResource(CardResource, i32),
        PlaceColony,
        MoveDelegete,
    }

    #[derive(Debug)]
    pub enum Language {
        English(String),
        Hungarian(String),
    }

    #[derive(Debug)]
    pub enum Color {
        Red,
        Green,
        Blue,
    }

    #[derive(Debug)]
    pub enum VictoryPoint {
        None,
        VP(i32),
        PerTag(i32, Tag, usize),
        PerResource(i32, CardResource, usize),
    }

    #[derive(Debug, PartialEq)]
    pub enum Tag {
        Building,
        Space,
        Science,
        Plant,
        Microbe,
        Animal,
        City,
        Earth,
        Jovian,
        Power,
        Event,
        Venus,
        Wild,
        Custom(String),
    }


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
        Custom(Box<dyn Fn(Game) -> bool>),
    }

    impl Debug for Requirement {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Requirement::Tag(tags) => {
                    let mut s = String::new();
                    for (tag, amount, minmax) in tags {
                        s.push_str(&format!("{} {:?} {:?}, ", amount, minmax, tag));
                    }
                    write!(f, "{}", s)
                },
                Requirement::Production(resource) => {
                    write!(f, "{:?}", resource)
                },
                Requirement::Tile(tile) => {
                    write!(f, "{}", tile)
                },
                Requirement::GlobalParameter(parameter, minmax) => {
                    write!(f, "{:?} {:?}", minmax, parameter)
                },
                Requirement::TR(amount, minmax) => {
                    write!(f, "{:?} {}", minmax, amount)
                },
                Requirement::CardResource(card_resource, amount) => {
                    write!(f, "{} {:?}", amount, card_resource)
                },
                Requirement::Colony(minmax) => {
                    write!(f, "{:?}", minmax)
                },
                Requirement::Chairman => {
                    write!(f, "Chairman")
                },
                Requirement::RulingParty(party) => {
                    write!(f, "{:?}", party)
                },
                Requirement::TwoPartyLeaders => {
                    write!(f, "TwoPartyLeaders")
                },
                Requirement::Custom(_) => {
                    write!(f, "Custom")
                },
            }
        }
    }

    #[derive(Debug)]
    pub enum TurmoilParty {
        Scientists,
        Unity,
        Reds,
        Greens,
        Kelvinists,
        MarsFirst,
    }

    #[derive(Debug)]
    pub enum MinMax {
        Min,
        Max,
    }

    #[derive(Debug)]
    pub enum GlobalParameter {
        Temperature(i32),
        Oxygen(usize),
        Ocean(usize),
        Venus(usize),
    }

    #[derive(Debug)]
    pub enum Resource {
        Money(usize),
        Steel(usize),
        Titanium(usize),
        Plant(usize),
        Energy(usize),
        Heat(usize),
    }

    #[derive(Debug)]
    pub enum CardResource {
        Microbe(usize),
        Animal(usize),
        Science(usize),
        Floaters(usize),
        Asteroid(usize),
        Data(usize),
        Radtiation(usize),
        Custom(String, Icon, usize),
    }

    ///! TODO
    #[derive(Debug)]
    pub enum Icon {
        Icon(String),
    }

}
