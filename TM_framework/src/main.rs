//! remove unreachable_code
#![allow(unreachable_code)]


use std::vec;

use TM_framework::*;



fn main() {
    println!("Hello, world! {}", VERSION);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();


    // max 99 card / expansion

    let orig_auc = Origin::Custom("Auction".to_string());
    let orig_intr = Origin::Custom("Intrica".to_string());

    let mut custom_cards = Vec::new();
    custom_cards.push(ProjectCard::new(
        "I01".to_string(),
        Color::Green,
        vec![Language::English("Company acquisition".to_string()), 
            Language::Hungarian("Cég felvásárlás".to_string())],
        45,
        vec![Language::Hungarian("Húzz 2-t a maradék cégbirodalom kártyák közül. Egyiket megkapod mint \"második céged\" a vagyonával és képességeivel együtt".to_string()),
            Language::English("Draw 2 cards from the remaining corporation cards, then choose 1 of them to become your \"second corporation\" with all its assets and abilities.".to_string())],
        ).set_vp(VictoryPoint::VP(3))
        .add_requirement(Requirement::Tag(vec![(Tag::Earth, 2, MinMax::Min)]))
        .add_origin(Origin::Custom("Intrica".to_string()))
    );
    custom_cards.push( ProjectCard::new(
        "#I02".to_string(),
        Color::Green,
        vec![Language::English("Aid from afar".to_string()), 
            Language::Hungarian("Utánküldött segítség".to_string())],
        17,
        vec![Language::Hungarian("Húzz 3-at a maradék prelude kártyák közül, majd játssz ki egyet.".to_string()),
            Language::English("Draw 3 cards from the remaining Prelude cards, then play 1 of them.".to_string())],
        ).add_origin(Origin::Custom("Intrica".to_string()))
        .add_requirement(Requirement::Tag(vec![(Tag::Earth, 1, MinMax::Min)]))
    );
    custom_cards.push( ProjectCard::new(
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
    custom_cards.push( ProjectCard::new(
        "#I04".to_string(),
        Color::Blue,
        vec![Language::English("Bio drone building".to_string()), 
            Language::Hungarian("Bio drón építés".to_string())],
        11,
        vec![Language::Hungarian("Akció: költs el 2 acél erőforrást, majd tegyél egy drónt erre a lapra VAGY költs el X (maximum annyi energia erőforrást, ahány drón van ezen a lapon), majd vegyél magadhoz 2X palánta erőforrást".to_string()),
            Language::English("Action: Spend 2 steel resources to add a drone to this card OR spend X energy (maximum drone number) to add 2X plants to your inventory.".to_string())],
        ).add_tags(vec![Tag::Building, Tag::Plant])
        .set_card_resource(CardResource::Custom("drone".to_string(), 0))
        .add_origin(Origin::Custom("Intrica".to_string()))
        .set_vp(VictoryPoint::VP(1))
    );
    // TODO separate this card into 2 cards
    custom_cards.push( ProjectCard::new(
        "#I05".to_string(),
        Color::Blue,
        vec![Language::English("Private army".to_string()), 
            Language::Hungarian("Magán hadsereg".to_string())],
        16,
        vec![Language::Hungarian("Csökkentsd a MC termelésed 4-el. Hatás: csak palánta erőforrást vehetnek el tőled és csak növényzet és óceán lapkát lehet másoknak helyezni a tiéd mellé".to_string()),
            Language::English("Decrease your MC production 4 step. Effect: only plant resources may be removed from you and only greenery and ocean tiles can be placed by others next to you.".to_string())],
        ).add_origin(Origin::Custom("Intrica".to_string()))
    );
    custom_cards.push( ProjectCard::new(
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
    custom_cards.push( ProjectCard::new(
        "#I07".to_string(),
        Color::Green,
        vec![Language::Hungarian("Nem etikus állatkísérletek".to_string()),
            Language::English("Unethical animal testing".to_string())],
        5,
        vec![Language::Hungarian("Minden játékos akinek van állat vagy ? ikonja felfedi 2 kártyáját, vásárolj meg ezekből bármennyit 1MC-ért".to_string()),
            Language::English("Each player with an animal or wild tag reveals 2 cards from their hand, then you may buy any number of them for 1MC each.".to_string())],
        ).add_origin(orig_intr).add_origin(orig_auc)
        .add_tag(Tag::Science)
        .set_vp(VictoryPoint::VP(-1))
    );
    custom_cards.push( ProjectCard::new(
        "#I08".to_string(),
        Color::Green,
        vec![Language::Hungarian("Közös kutatás".to_string()),
            Language::English("Joint research".to_string())],
        6,
        vec![Language::Hungarian("Minden játékos rajtad kívül húz egy kártyát. Ezt vagy megveszi 3MC-ért, vagy a kezedbe adja. Ezután egészítsd ki 5-re a kapott kártya mennyiséget, majd ebből egyet a kezedbe vehetsz, dobd el a többit.".to_string()),
            Language::English("Each player draws a card, then either buys it for 3MC or gives it to you. Then draw cards untill you have 5 in your hand with the recived ones, add 1 to your hand and discard the rest.".to_string())],
        ).add_origin(Origin::Custom("Intrica".to_string()))
        .add_tags(vec![Tag::Science, Tag::Science])
    );
    println!("{:?}", custom_cards);
    
    custom_cards.push( ProjectCard::new(
        "#I09".to_string(),
        Color::Green,
        vec![Language::Hungarian("Érzékeny importált állatok".to_string()),
            Language::English("Sensitive imported animals".to_string())],
        15,
        vec![Language::Hungarian("".to_string()),
            Language::English("".to_string())],
        ).add_origin(Origin::Custom("Intrica".to_string()))
        .add_requirement(Requirement::GlobalParameter(GlobalParameter::Ocean(9), MinMax::Min))
        .add_requirement(Requirement::GlobalParameter(GlobalParameter::Oxygen(14), MinMax::Min))
        .add_requirement(Requirement::GlobalParameter(GlobalParameter::Temperature(8), MinMax::Min))
        .set_vp(VictoryPoint::PerResource(2, CardResource::Animal(1)))
    );
    custom_cards.push( ProjectCard::new(
        "#I10".to_string(),
        Color::Blue,
        vec![Language::Hungarian("Magán tó".to_string()),
            Language::English("Private lake".to_string())],
        18,
        vec![Language::Hungarian("Helyezz el egy óceánt egy NEM FENNTARTOTT helyre, majd HELYEZD RÁ EGY JELÖLŐDET. Csak te építkezhetsz mellette.".to_string()),
            Language::English("Place an ocean tile on a NON RESERVED land area, then PLACE YOUR MARKER ON IT. Only you may build next to it.".to_string())],
        ).add_origin(Origin::Custom("Intrica".to_string()))
    );
    custom_cards.push( ProjectCard::new(
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
        .add_requirement(Requirement::Tag(vec![(Tag::Jovian, 1, MinMax::Min)]))
    );
    custom_cards.push( ProjectCard::new(
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
    // 

    custom_cards.push( ProjectCard::new(
        "#I16".to_string(), 
        Color::Blue, 
        vec![Language::Hungarian("Kiskedvenc klónozás".to_string())], 
        16, 
        vec![
            Language::Hungarian("Kötelező Akció: vegyél le egy mikróbát, egy palánta és egy energia erőforrást. Tegyél egy állatot bármely kártyádra. -1 VP minden állatod után ezen a kártyán".to_string())]
        ).add_origin(Origin::Custom("Intrica".to_string())).add_origin(orig_auc)
        .add_requirement(Requirement::Tag(vec![(Tag::Science, 2, MinMax::Min)]))
        .add_on_card_actions(vec![
            OnCardAction::ModifyProduction(Resource::Plant(3)),
            OnCardAction::ModifyProduction(Resource::Energy(-1))])
        .set_card_resource(CardResource::Animal(2))
        .set_vp(VictoryPoint::PerResource(-1, CardResource::Animal(1)))
        .add_motivational_quotes(vec![
            Language::Hungarian("A klónozáshoz csupán 2 dolog kell. Energia és biomassza. Nagyon sok biomassza!".to_string()),
            Language::English("You only need two things for cloning. Energy and biomass. A lot of biomass!".to_string())])

    );

    custom_cards.push( ProjectCard::new(
        "#I17".to_string(),
        Color::Blue,
        ///! find a better name
        vec![Language::Hungarian("Eget rengető tudományos felfedezés".to_string()), 
            Language::English("Groudbreaking scientific discovery".to_string())],
        25,
        vec![Language::Hungarian("Hatás: Ha több mint 15 tudomány ikonod van, azonnal megnyered a játékot.".to_string()),
            Language::English("Effect: If you have more than 15 science tags, you win the game immediately.".to_string())],
        ).add_origin(Origin::Custom("Intrica".to_string())).add_origin(orig_auc)
        .add_tag(Tag::Custom("Wonder".to_string()))
        .set_vp(VictoryPoint::PerTag(1, Tag::Science, 2))
        .add_requirement(Requirement::Tag(vec![(Tag::Science, 10, MinMax::Max)]))
    );
    custom_cards.push( ProjectCard::new(
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

    custom_cards.push(ProjectCard::new(
        "#I22".to_owned(), 
        Color::Blue, 
        vec![], 
        12,
        vec![Language::Hungarian("Amikor BÁRMILYEN kártyát húzol, húzz egyel többet, de dobj el egyet a húzottak közül, mielőtt a kezedbe vennéd amit kiválsztottál".to_string()),
            Language::English("When you draw ANY cards, draw 1 extra, then discard 1 from the drawn, before you buy / take them".to_string())]
        ).add_origin(orig_intr).add_origin(Origin::CorporateEra)
        .add_tag(Tag::Science)
    );

    // __________________________________________________________________6 cards __________________________________________________________________________
    // _____________________________________________________________Turmoil extended_______________________________________________________________________
    // one more of these cards with other extensions:
    // scientist, kelvinist, unity
    // type of cards: base(extensions)
    // mars:      G 1(1), B 1,    R (1)
    // scientist: G 1,    B 1,    R (2)
    // unity:     G 2,    B (1),  R (1)
    // green:     G 1,    B 1(1), R 0
    // reds:      G 1(1), B 0,    R 1
    // kelvinist: G 2,    B (1),  R (1)

    // mars:      G 4,    B 3,    R 1
    // scientist: G 5,    B 3,    R 0
    // unity:     G 4,    B 3,    R 1
    // green:     G 5,    B 2,    R 2
    // reds:      G 4,    B 4,    R 1
    // kelvinist: G 4,    B 3,    R 1

    // green, red: moon 
    custom_cards.push( ProjectCard::new(
        "#L01".to_string(),
        Color::Green,
        vec![Language::Hungarian("".to_string()),
            Language::English("".to_string())],
        todo!(),
        vec![Language::Hungarian("".to_string()),
            Language::English("".to_string())],
        )
    );
    custom_cards.push( ProjectCard::new(
        "#L02".to_string(),
        Color::Green,
        vec![Language::Hungarian("".to_string()),
            Language::English("".to_string())],
        todo!(),
        vec![Language::Hungarian("".to_string()),
            Language::English("".to_string())],
        )
    );
    custom_cards.push( ProjectCard::new(
        "#L03".to_string(),
        Color::Green,
        vec![Language::Hungarian("".to_string()),
            Language::English("".to_string())],
        todo!(),
        vec![Language::Hungarian("".to_string()),
            Language::English("".to_string())],
        )
    );
    custom_cards.push( ProjectCard::new(
        "#L04".to_string(),
        Color::Blue,
        vec![Language::Hungarian("Ültetés törvény".to_string()),
            Language::English("Goverment mandated planting".to_string())],
        todo!(),
        vec![Language::Hungarian("Hatás:_Valahányszor lapot helyezel el A MARSON, kapsz egy palántát.".to_string()),
            Language::English("Effect: Whenever you place a tile on MARS, gain a plant.".to_string())],
        ).add_origin(Origin::Turmoil).add_origin(Origin::Custom("Ligvigfui extention".to_string()))
        .add_requirement(Requirement::RulingParty(TurmoilParty::Greens))
    );
    custom_cards.push( ProjectCard::new(
        "#L05".to_string(),
        Color::Blue,
        vec![Language::Hungarian("Építkezési engedély".to_string()),
            Language::English("Land usage permit".to_string())],
        todo!(),
        vec![Language::Hungarian("Hatás: Minden épület 2 Mc-el kevesebbe kerül, Növeld a Mc termelésed 1-el".to_string()),
            Language::English("Effect: All buildings cost 2MC less, Increase your MC production 1 step.".to_string())],
        ).add_origin(Origin::Turmoil).add_origin(Origin::Custom("Ligvigfui extention".to_string()))
        .add_requirement(Requirement::RulingParty(TurmoilParty::MarsFirst))
    );
    custom_cards.push( ProjectCard::new(
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