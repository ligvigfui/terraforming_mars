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
        Auction,
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
        motivational_quote: Option<Vec<Language>>,
        origin: Vec<Origin>
    }

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
                motivational_quote: None,
                origin: Vec::new(),
            }
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
        active_player: i32,
        generation: i32,
        map: String,
        temperature: i32,
        oxygen: i32,
        ocean: i32,
        venus: i32,
        milestones: Vec<Card>,
        awards: Vec<Card>,
        phase: String,
        history: String,
    }

    #[derive(Debug)]
    ///! still not good need to change milestone, award, action
    struct Player {
        id: i32,
        name: String,
        cards: Vec<Card>,
        production: Vec<Resource>,
        resources: Vec<Resource>,
        vp: i32,
        terraform_rating: i32,
        corporation: Vec<Corporation>,
        prelude: Vec<Prelude>,
        milestones: Vec<Card>,
        awards: Vec<Card>,
        actions: i32,
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

    #[derive(Debug)]
    pub enum OnCardAction {
        // move card from research to hand
        BuyCard(i32),
        // draw random card from deck
        DrawCard(i32),
        // move card from research or hand to discard
        DiscardCard(i32),
        ModifyResources(Resource),
        ModifyProduction(Resource),
        MustRemoveFromOtherPlayersResources(Resource), 
        MustRemoveFromOtherPlayersProduction(Resource),
        RemoveFromOtherPlayersResources(Resource),
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
        PerTag(i32, Tag, i32),
        PerResource(i32, CardResource, i32),
    }

    #[derive(Debug)]
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
    }

    pub enum Requirement {
        Tag(Vec<(Tag, i32)>),
        Production(Resource),
        Tile(String),
        GlobalParameter(GlobalParameter, MinMax),
        TR(i32 , MinMax),
        CardResource(CardResource, i32),
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
                    for (tag, amount) in tags {
                        s.push_str(&format!("{} {:?}, ", amount, tag));
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
        Oxygen(i32),
        Ocean(i32),
        Venus(i32),
    }

    #[derive(Debug)]
    pub enum Resource {
        Money(i32),
        Steel(i32),
        Titanium(i32),
        Plant(i32),
        Energy(i32),
        Heat(i32),
    }

    #[derive(Debug)]
    pub enum CardResource {
        Microbe(i32),
        Animal(i32),
        Science(i32),
        Asteroid(i32),
        Floaters(i32),
        Custom(String, Icon, i32),
    }

    ///! TODO
    #[derive(Debug)]
    pub enum Icon {
        Icon(String),
    }

}
