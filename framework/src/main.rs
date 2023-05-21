//disable warn dead code
#![allow(dead_code)]

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
enum Origin{
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
struct Card {
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
    fn new(id: String, color: Color, name: Vec<Language>, cost: i32, effects: Vec<Language>) -> Card {
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
    fn set_requironment(mut self, requirement: Requirement)-> Card{
        self.requirements = Some(requirement);
        self
    }
    fn set_vp(mut self, vp: VictoryPoint)-> Self{
        self.vp = vp;
        self
    }
    fn add_tag(mut self, tag: Tag)-> Self{
        self.tags.push(tag);
        self
    }
    fn add_tags(mut self, tags: Vec<Tag>)-> Self{
        self.tags.extend(tags);
        self
    }
    fn set_card_resource(mut self, card_resource: CardResource)->Self{
        self.card_resource = Some(card_resource);
        self
    }
    fn set_bets_time_to_get(mut self, bets_time_to_get: Option<(Usefullness, Usefullness, Usefullness)>)->Self{
        self.bets_time_to_get = bets_time_to_get;
        self
    }
    fn add_motivational_quote(mut self, motivational_quote: Language)-> Self{
        let mot_quote = motivational_quote;
        if let Some(ref mut motivational_quote) =self.motivational_quote {
            motivational_quote.push(mot_quote);
        } else {
            self.motivational_quote = Some(vec![mot_quote]);
        }
        self
    }
    fn add_motivational_quotes(mut self, motivational_quotes: Vec<Language>)-> Self{
        let mot_quotes = motivational_quotes;
        if let Some(ref mut motivational_quote) =self.motivational_quote {
            motivational_quote.extend(mot_quotes);
        } else {
            self.motivational_quote = Some(mot_quotes);
        }
        self
    }
    fn add_origin(mut self, origin: Origin)->Self{
        self.origin.push(origin);
        self
    }
}
#[derive(Debug)]
enum Usefullness{
    Great,
    Ok,
    Bad,
}





#[derive(Debug)]
///! still not good need to change milestone, award, phase, history, include turmoil, colonies
struct Game {
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
enum OnCardAction {
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
enum Language {
    English(String),
    Hungarian(String),
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
enum VictoryPoint {
    None,
    VP(i32),
    PerTag(i32, Tag, i32),
    PerResource(i32, CardResource, i32),
}

#[derive(Debug)]
enum Tag {
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

enum Requirement {
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
enum TurmoilParty {
    Scientists,
    Unity,
    Reds,
    Greens,
    Kelvinists,
    MarsFirst,
}

#[derive(Debug)]
enum MinMax {
    Min,
    Max,
}

#[derive(Debug)]
enum GlobalParameter {
    Temperature(i32),
    Oxygen(i32),
    Ocean(i32),
    Venus(i32),
}

#[derive(Debug)]
enum Resource {
    Money(i32),
    Steel(i32),
    Titanium(i32),
    Plant(i32),
    Energy(i32),
    Heat(i32),
}

#[derive(Debug)]
enum CardResource {
    Microbe(i32),
    Animal(i32),
    Science(i32),
    Asteroid(i32),
    Floaters(i32),
    Custom(String, Icon, i32),
}

///! TODO
#[derive(Debug)]
enum Icon {
    Icon(String),
}

fn main() {

    let mut custom_cards = Vec::new();
    custom_cards.push(Card::new(
        "I01".to_string(),
        Color::Green,
        vec![Language::English("Company acquisition".to_string()), 
            Language::Hungarian("Cég felvásárlás".to_string())],
        45,
        vec![Language::Hungarian("Húzz 2-t a maradék cégbirodalom kártyák közül. Egyiket megkapod mint \"második céged\" a vagyonával és képességeivel együtt".to_string()),
            Language::English("Draw 2 cards from the remaining corporation cards, then choose 1 of them to become your \"second corporation\" with all its assets and abilities.".to_string())],
        ).set_vp(VictoryPoint::VP(3))
        .set_requironment(Requirement::Tag(vec![(Tag::Earth, 1)]))
        .add_origin(Origin::Custom("Intrica".to_string()))
    );
    custom_cards.push(Card::new(
        "#I02".to_string(),
        Color::Green,
        vec![Language::English("Aid from afar".to_string()), 
            Language::Hungarian("Utánküldött segítség".to_string())],
        17,
        vec![Language::Hungarian("Húzz 3-at a maradék prelude kártyák közül, majd játssz ki egyet.".to_string()),
            Language::English("Draw 3 cards from the remaining Prelude cards, then play 1 of them.".to_string())],
        ).set_requironment(Requirement::Tag(vec![(Tag::Earth, 1)]))
        .add_origin(Origin::Custom("Intrica".to_string()))
    );
    custom_cards.push(Card::new(
        "#I03".to_string(),
        Color::Green,
        vec![Language::English("Célzott kutatás".to_string()), 
            Language::Hungarian("Targeted research".to_string())],
        7,
        vec![Language::Hungarian("Keress egy általad választott jelzésű kártyát a pakliból, majd vedd a kezedbe! A többi lapot dobd el.".to_string()),
            Language::English("Search the deck for a card with a tag of your choice, and add it to your hand. Discard the rest.".to_string())],
        ).set_requironment(Requirement::Tag(vec![(Tag::Earth, 1)]))
        .add_tag(Tag::Science)
        .add_origin(Origin::Custom("Intrica".to_string()))
    );
    custom_cards.push(Card::new(
        "#I04".to_string(),
        Color::Blue,
        vec![Language::English("Bio drone building".to_string()), 
            Language::Hungarian("Bio drón építés".to_string())],
        11,
        vec![Language::Hungarian("Akció: költs el 2 acél erőforrást, majd tegyél egy drónt erre a lapra VAGY költs el X (maximum annyi energia erőforrást, ahány drón van ezen a lapon), majd vegyél magadhoz 2X palánta erőforrást".to_string()),
            Language::English("Action: Spend 2 steel resources to add a drone to this card OR spend X energy (maximum drone number) to add 2X plants to your inventory.".to_string())],
        ).set_requironment(Requirement::Tag(vec![(Tag::Earth, 1)]))
        .add_tags(vec![Tag::Building, Tag::Plant])
        .set_card_resource(CardResource::Custom("drone".to_string(), Icon::Icon("drone".to_string()), 0))
        .add_origin(Origin::Custom("Intrica".to_string()))
        .set_vp(VictoryPoint::VP(1))
    );
    // TODO separate this card into 2 cards
    custom_cards.push(Card::new(
        "#I05".to_string(),
        Color::Blue,
        vec![Language::English("Private army".to_string()), 
            Language::Hungarian("Magán hadsereg".to_string())],
        16,
        vec![Language::Hungarian("Csökkentsd a MC termelésed 4-el. Hatás: csak palánta erőforrást vehetnek el tőled és csak növényzet és óceán lapkát lehet másoknak helyezni a tiéd mellé".to_string()),
            Language::English("Decrease your MC production 4 step. Effect: only plant resources may be removed from you and only greenery and ocean tiles can be placed by others next to you.".to_string())],
        ).add_origin(Origin::Custom("Intrica".to_string()))
    );
    custom_cards.push(Card::new(
        "#I06".to_string(),
        Color::Red,
        vec![Language::English("Delivery error".to_string()), 
            Language::Hungarian("Téves szállítás".to_string())],
        1,
        vec![Language::Hungarian("Cseréld fel egy tetszőleges erőforrásod egészét egy társaddal (pl 20 MC - 6MC -> 6MC - 20 MC)".to_string()),
            Language::English("Exchange all of your resources of your chosing for all of another player's resources.".to_string())],
        ).add_origin(Origin::Custom("Intrica".to_string()))
        .set_vp(VictoryPoint::VP(-2))
        .add_tag(Tag::Event)
    );
    // __________________________________________________________________6 cards __________________________________________________________________________
    custom_cards.push(Card::new(
        "#I07".to_string(),
        Color::Green,
        vec![Language::Hungarian("Nem etikus állatkísérletek".to_string()),
            Language::English("Unethical animal testing".to_string())],
        5,
        vec![Language::Hungarian("Minden játékos akinek van állat vagy ? ikonja felfedi egy kártyáját, vásárolj meg ezekből bármennyit 1MC-ért".to_string()),
            Language::English("Each player with an animal or wild tag reveals a card from their hand, then you may buy any number of them for 1MC each.".to_string())],
        ).add_origin(Origin::Custom("Intrica".to_string()))
        .add_tag(Tag::Science)
        .set_vp(VictoryPoint::VP(-1))
    );
    custom_cards.push(Card::new(
        "#I08".to_string(),
        Color::Green,
        vec![Language::Hungarian("Közös kutatás".to_string()),
            Language::English("Joint research".to_string())],
        6,
        vec![Language::Hungarian("Minden játékos rajtad kívül húz egy kártyát. Ezt vagy megveszi 3MC-ért, vagy a kezedbe adja. Ezután húzz (5-kapott) kártyát, majd ebből egyet a kezedbe vehetsz, dobd el a többit.".to_string()),
            Language::English("Each player draws a card, then either buys it for 3MC or gives it to you. Then draw (5-recived) cards, chose 1 and discard the rest.".to_string())],
        ).add_origin(Origin::Custom("Intrica".to_string()))
        .add_tags(vec![Tag::Science, Tag::Science])
    );
    custom_cards.push(Card::new(
        "#I09".to_string(),
        Color::Green,
        vec![Language::Hungarian("".to_string()),
            Language::English("".to_string())],
        todo!(),
        vec![Language::Hungarian("".to_string()),
            Language::English("".to_string())],
        ).add_origin(Origin::Custom("Intrica".to_string()))
    );
    custom_cards.push(Card::new(
        "#I10".to_string(),
        Color::Blue,
        vec![Language::Hungarian("Magán tó".to_string()),
            Language::English("Private lake".to_string())],
        18,
        vec![Language::Hungarian("Helyezz el egy óceánt egy NEM FENNTARTOTT helyre, majd HELYEZD RÁ EGY JELÖLŐDET. Csak te építkezhetsz mellette.".to_string()),
            Language::English("Place an ocean tile on a NON RESERVED land area, then PLACE YOUR MARKER ON IT. Only you may build next to it.".to_string())],
        ).add_origin(Origin::Custom("Intrica".to_string()))
    );
    custom_cards.push(Card::new(
        "#I11".to_string(),
        Color::Blue,
        vec![Language::Hungarian("Bázis a Titánon".to_string()),
            Language::English("Base on Titan".to_string())],
        21,
        vec![Language::Hungarian("Hatás: Ha kijátszol egy űrkártyát, helyezz egy titán erőforrást erre a lapra. Ha kijátszol egy Jupiter kártyát, akkor a titán erőforrásokat erről a lapról is használhatod. 2 VP minden Jupiter ikonért amit birtokolsz.".to_string()),
            Language::English("Effect: When you play a space card, place a titan resourse to THIS card. When you play a jovian you can use titan from this card like normal. 2 VP per Jovian tag you have.".to_string())],
        ).add_origin(Origin::Custom("Intrica".to_string()))
        .add_tag(Tag::Space)
        .set_vp(VictoryPoint::PerTag(2, Tag::Jovian, 1))
        .set_requironment(Requirement::Tag(vec![(Tag::Jovian, 1)]))
    );
    custom_cards.push(Card::new(
        "#I12".to_string(),
        Color::Red,
        vec![Language::Hungarian("Uno revers".to_string()),
            Language::English("Uno revers".to_string())],
        todo!(),
        vec![Language::Hungarian("Játszd ki ezt a kártyát a körödön kívül, amikor valaki végrehajttat veled egy akciót. A tervezett negatívumot neki kell elszenvednie. (ha nem képes ő elvégezni, akkor te választod ki, hogy ki legyen az)".to_string()),
            Language::English("Play this card outside of your turn when someone performs an action on you. The intended negative effect is applied to them instead. (if they are unable to perform the action, you choose who it is applied to)".to_string())],
        ).add_origin(Origin::Custom("Intrica".to_string()))
        .add_tag(Tag::Event)
    );
    // __________________________________________________________________6 cards __________________________________________________________________________
    // _____________________________________________________________Ligvigfui extension____________________________________________________________________
    // not green: kelvinists, unity
    // not red: reds
    // not blue: marsfirst, greens, scientists
    custom_cards.push(Card::new(
        "#L01".to_string(),
        Color::Green,
        vec![Language::Hungarian("".to_string()),
            Language::English("".to_string())],
        todo!(),
        vec![Language::Hungarian("".to_string()),
            Language::English("".to_string())],
        )
    );
    custom_cards.push(Card::new(
        "#L02".to_string(),
        Color::Green,
        vec![Language::Hungarian("".to_string()),
            Language::English("".to_string())],
        todo!(),
        vec![Language::Hungarian("".to_string()),
            Language::English("".to_string())],
        )
    );
    custom_cards.push(Card::new(
        "#L03".to_string(),
        Color::Green,
        vec![Language::Hungarian("".to_string()),
            Language::English("".to_string())],
        todo!(),
        vec![Language::Hungarian("".to_string()),
            Language::English("".to_string())],
        )
    );
    custom_cards.push(Card::new(
        "#L04".to_string(),
        Color::Blue,
        vec![Language::Hungarian("Ültetés törvény".to_string()),
            Language::English("Goverment mandated planting".to_string())],
        todo!(),
        vec![Language::Hungarian("Hatás:_Valahányszor lapot helyezel el A MARSON, kapsz egy palántát.".to_string()),
            Language::English("Effect: Whenever you place a tile on MARS, gain a plant.".to_string())],
        ).add_origin(Origin::Turmoil).add_origin(Origin::Custom("Ligvigfui extention".to_string()))
        .set_requironment(Requirement::RulingParty(TurmoilParty::Greens))
    );
    custom_cards.push(Card::new(
        "#L05".to_string(),
        Color::Blue,
        vec![Language::Hungarian("Építkezési engedély".to_string()),
            Language::English("Land usage permit".to_string())],
        todo!(),
        vec![Language::Hungarian("Hatás: Minden épület 2 Mc-el kevesebbe kerül, Növeld a Mc termelésed 1-el".to_string()),
            Language::English("Effect: All buildings cost 2MC less, Increase your MC production 1 step.".to_string())],
        ).add_origin(Origin::Turmoil).add_origin(Origin::Custom("Ligvigfui extention".to_string()))
        .set_requironment(Requirement::RulingParty(TurmoilParty::MarsFirst))
    );
    custom_cards.push(Card::new(
        "#L06".to_string(),
        Color::Red,
        vec![Language::Hungarian("".to_string()),
            Language::English("".to_string())],
        todo!(),
        vec![Language::Hungarian("".to_string()),
            Language::English("".to_string())],
        )
    );

}