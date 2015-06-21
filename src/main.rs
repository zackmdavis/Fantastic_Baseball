struct PositionPlayer<'a> {
    uniform_number: u8,
    nonfirst_name: &'a str,
    last_name: &'a str,
    batting_average: f64,
    on_base_percentage: f64,
    slugging_percentage: f64,
}

struct Pitcher<'a> {
    uniform_number: u8,
    nonfirst_name: &'a str,
    last_name: &'a str,
    walks_plus_hits_per_inning_pitched: f64,
    // the designated hitter rule is contrary to the operation of the
    // moral law
    batting_average: f64,
}

// TODO: common player trait?
// trait PlayerNature {}
// impl PlayerNature for PositionPlayer {}
// impl PlayerNature for Pitcher {}

struct Team<'a> {
    city: &'a str,
    name: &'a str,
    pitcher: Pitcher<'a>,
    catcher: PositionPlayer<'a>,
    first_baseperson: PositionPlayer<'a>,  // gender-neutral language
    second_baseperson: PositionPlayer<'a>,
    third_baseperson: PositionPlayer<'a>,
    shortstop: PositionPlayer<'a>,
    leftfielder: PositionPlayer<'a>,
    centerfielder: PositionPlayer<'a>,
    rightfielder: PositionPlayer<'a>,
}

struct GameState<'a> {
    inning: u8,
    home_team: Team<'a>,
    away_team: Team<'a>,

    home_score: &'a mut u8,
    away_score: &'a mut u8,

    // XXX TODO FIXME: because the designated hitter rule is contrary
    // to the operation of the moral law, both Pitchers and
    // PositionPlayers should be able to be on base (both structs
    // implementing a common Player trait??).
    first_base: &'a mut Option<PositionPlayer<'a>>,
    second_base: &'a mut Option<PositionPlayer<'a>>,
    third_base: &'a mut Option<PositionPlayer<'a>>,

    balls: &'a mut u8,
    strikes: &'a mut u8,
    outs: &'a mut u8,
}


fn main() {
    // TODO
}
