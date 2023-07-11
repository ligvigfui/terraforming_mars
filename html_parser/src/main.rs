//this crate imports cards from the official terraforming mars website and saves them as json files

use std::vec;

use TM_framework::*;




trait FromHtml {
    fn from_html(html: &str) -> Self;
}
impl FromHtml for ProjectCard {
    fn from_html(html: &str) -> Self {
        let cost = html.find("price>").unwrap();
        let end = html[cost..].find("<").unwrap();
        let cost = html[cost + 6..cost + end].parse::<i32>().unwrap();
        let card = ProjectCard::new(
            "id".to_string(),
            Color::Green,
            vec![],
            cost,
            vec![],
        );
        card
    }
}
fn extract(html: &str, start: &str, end: &str) -> String {
    let start = html.find(start).unwrap();
    let end = html[start..].find(end).unwrap();
    html[start..end].to_string()
}

fn main() {
    println!("Hello, world!");
}


//create tests
#[cfg(test)]
mod tests {
    use super::*;
        


    fn one_card() -> String {
        "<!--245-->
            <li onclick=\"getClickedCard();\" class=\"filterDiv automated  venusNext noneTag reqs show\" data-jovian=\"1\" data-earth=\"1\" data-venustag=\"1\">
                <div class=\"title background-color-automated \">Solarnet</div>
                <div class=\"price\">7</div>
                <div class=\"number\">#245</div>
                <div class=\"venus-icon project-icon\"></div>
                <div class=\"content \">
                  <div class=\"points points-big\">1</div>
                  <div class=\"requirements\"> Venus Earth Jovian</div>
                  <div class=\"resource card\"></div> <div class=\"resource card\"></div>
                  <div class=\"description\">
                    (Requires Venus, Earth and Jovian tags. Draw 2 cards).
                  </div>
                </div>
            </li>"
            .to_string()
    }

    #[test]
    fn test_card_from_html(){
        let card = one_card();
        let card = ProjectCard::from_html(&card);
        let expected_card = ProjectCard::new(
            "#245".to_string(),
            Color::Green,
            vec![Language::English("Solarnet".to_string())],
            7,
            vec![Language::English("(Requires Venus, Earth and Jovian tags. Draw 2 cards).".to_string())],
        ).add_origin(Origin::VenusNext)
        .add_requirement(Requirement::Tag(vec![
            (Tag::Venus, 1, MinMax::Min),
            (Tag::Earth, 1, MinMax::Min),
            (Tag::Jovian, 1, MinMax::Min),
        ]))
        .add_on_card_action(OnCardAction::DrawCard(2))
        .set_vp(VictoryPoint::VP(1));
        assert_eq!(card, expected_card);
    }

}
