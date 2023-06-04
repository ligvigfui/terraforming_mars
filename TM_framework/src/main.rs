// Purpose: main file of the TM_framework
//import dependencies

use TM_framework::cards::{Card, Color, Language, VictoryPoint, Tag, Requirement, CardResource, Icon, Origin, Game, MinMax, TurmoilParty};

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
        .set_requironment(Requirement::Tag(vec![(Tag::Earth, 1, MinMax::Min)]))
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
        ).add_origin(Origin::Custom("Intrica".to_string()))
    );
    custom_cards.push(Card::new(
        "#I03".to_string(),
        Color::Green,
        vec![Language::English("Célzott kutatás".to_string()), 
            Language::Hungarian("Targeted research".to_string())],
        7,
        vec![Language::Hungarian("Keress egy általad választott jelzésű kártyát a pakliból, majd vedd a kezedbe! A többi lapot dobd el.".to_string()),
            Language::English("Search the deck for a card with a tag of your choice, and add it to your hand. Discard the rest.".to_string())],
        ).add_tag(Tag::Science)
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
        ).add_tags(vec![Tag::Building, Tag::Plant])
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
    println!("{:?}", custom_cards);
    
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
        .set_requironment(Requirement::Tag(vec![(Tag::Jovian, 1, MinMax::Min)]))
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
    
    custom_cards.push(Card::new(
        "#I17".to_string(),
        Color::Blue,
        ///! find a better name
        vec![Language::Hungarian("Eget rengető tudományos felfedezés".to_string()), 
            Language::English("Groudbreaking scientific discovery".to_string())],
        25,
        vec![Language::Hungarian("Hatás: Ha több mint 15 tudomény ikonod van, azonnal megnyered a játékot.".to_string()),
            Language::English("Effect: If you have more than 15 science tags, you win the game immediately.".to_string())],
        ).add_origin(Origin::Custom("Intrica".to_string())).add_origin(Origin::Auction)
        .add_tag(Tag::Custom("Wonder".to_string()))
        .set_vp(VictoryPoint::PerTag(1, Tag::Science, 2))
        .set_requironment(Requirement::Custom(Box::new(|game: Game| {
            match game.currentPlayer().tags().iter().filter(|(tag, _)| *tag == Tag::Science).next(){
                Some((_, amount)) => *amount <= 10,
                None => true,
            }
        })))
        
    );
    custom_cards.push(Card::new(
        "#I18".to_string(),
        Color::Red,
        vec![Language::Hungarian("".to_string()),
            Language::English("".to_string())],
        todo!(),
        vec![Language::Hungarian("".to_string()),
            Language::English("".to_string())],
        ).add_origin(Origin::Custom("Intrica".to_string()))
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