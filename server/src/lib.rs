use TM_framework::*;

pub mod user;

pub struct AllRules {
    extra_rules: ExtraRules,
    rules: Rules,
}

pub struct ExtraRules {
    // remove 10 coins per generation by default.

    // true => +1 coins and the first some at the end of the season get some. maybe leagues?
    play_for_ranked: bool,
    // true => +10 coins and the coins are redestrebuted in accordance to the US electoral vote distrebution system.
    // coin sum (- 5?*player thats for me) 
    play_for_money: bool,
}