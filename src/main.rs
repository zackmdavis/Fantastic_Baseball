extern crate rand;

struct PositionPlayer<'a> {
    uniform_number: u8,
    nonlast_name: &'a str,
    last_name: &'a str,
    batting_average: f64,
    on_base_percentage: f64,
    slugging_percentage: f64,
}

struct Pitcher<'a> {
    uniform_number: u8,
    nonlast_name: &'a str,
    last_name: &'a str,
    walks_plus_hits_per_inning_pitched: f64,
    // the designated hitter rule is contrary to the operation of the
    // moral law
    batting_average: f64,
}

enum AtBatOutcome {
    FieldOut,
    KSwinging,
    KLooking,
    BaseOnBalls,
    HitByPitch,
    Single,
    Double,
    Triple,
    HomeRun,
    InsideHomeRun,
}

trait PlayerNature {
    fn at_bat(&self, pitcher: &Pitcher) -> AtBatOutcome;
}

impl<'a> PlayerNature for PositionPlayer<'a> {
    // XXX CODE DUPLICATION: we'd prefer to just define this function
    // once in the `trait PlayerNature` block, but I'm not sure how to
    // assure the compiler that both `Pitcher` and `PositionPlayer` have
    // a `batting_average` ("attempted access of field `batting_average`
    // on type `&Self`, but no field with that name was found")
    fn at_bat(&self, pitcher: &Pitcher) -> AtBatOutcome {
        if self.batting_average < rand::random() {
            // (These static outcomes are clearly a placeholder for
            // TODO more sophisticated weighted outcome generation to be
            // implemented later.)
            AtBatOutcome::Single
        } else {
            AtBatOutcome::FieldOut
        }
    }
}
impl<'a> PlayerNature for Pitcher<'a> {
    fn at_bat(&self, pitcher: &Pitcher) -> AtBatOutcome {
        if self.batting_average < rand::random() {
            AtBatOutcome::Single
        } else {
            AtBatOutcome::FieldOut
        }
    }
}

struct Team<'a> {
    city: &'a str,
    name: &'a str,

    pitcher: Pitcher<'a>,
    catcher: PositionPlayer<'a>,
    first_baseperson: PositionPlayer<'a>,  // gender-neutral language ⚤
    second_baseperson: PositionPlayer<'a>,
    third_baseperson: PositionPlayer<'a>,
    shortstop: PositionPlayer<'a>,
    leftfielder: PositionPlayer<'a>,
    centerfielder: PositionPlayer<'a>,
    rightfielder: PositionPlayer<'a>,

    lineup: [u8; 8], // make this length 9 when we figure out how to
                     // solve our generic player problem
}

// XXX "error: mismatched types: [..] (expected type parameter, found
// struct `PositionPlayer`)"
//
// uhhh, are we not allowed to return generic types?!
// fn position_number_to_player<P: PlayerNature>(team: Team,
//                                               position_number: u8) -> P {
fn position_number_to_player(team: Team, position_number: u8) -> PositionPlayer {
    match position_number {
        // 1 =>  team.pitcher,
        2 =>  team.catcher,
        3 =>  team.first_baseperson,
        4 =>  team.second_baseperson,
        5 =>  team.third_baseperson,
        6 =>  team.shortstop,
        7 =>  team.leftfielder,
        8 =>  team.centerfielder,
        9 =>  team.rightfielder,
        _ => panic!("fake position number")
    }
}

struct GameState<'a> {
    inning: u8,
    home_team: Team<'a>,
    away_team: Team<'a>,

    home_score: u8,
    away_score: u8,

    home_to_bat: u8,
    away_to_bat: u8,

    // XXX TODO FIXME: because the designated hitter rule is contrary
    // to the operation of the moral law, both Pitchers and
    // PositionPlayers should be able to be on base (both structs
    // implementing a common Player trait??).
    first_base: Option<PositionPlayer<'a>>,
    second_base:Option<PositionPlayer<'a>>,
    third_base: Option<PositionPlayer<'a>>,

    balls: u8,
    strikes: u8,
    outs: u8,
}


fn main() {
    let away = Team {
        city: "San Francisco",
        name: "Giants",
        pitcher: Pitcher {
            uniform_number: 22, nonlast_name: "Madison", last_name: "Bumgarner",
            walks_plus_hits_per_inning_pitched: 2.0, batting_average: 0.1
        },
        catcher: PositionPlayer {
            uniform_number: 0, nonlast_name: "Buster", last_name: "Posey",
            batting_average: 0.311, on_base_percentage: 0.364,
            slugging_percentage: 0.490
        },
        first_baseperson: PositionPlayer {
            uniform_number: 1, nonlast_name: "Brandon", last_name: "Belt",
            batting_average: 0.243, on_base_percentage: 0.306,
            slugging_percentage: 0.449
        },
        second_baseperson: PositionPlayer {
            uniform_number: 2, nonlast_name: "Joe", last_name: "Panik",
            batting_average: 0.305, on_base_percentage: 0.343,
            slugging_percentage: 0.368
        },
        third_baseperson: PositionPlayer {
            uniform_number: 3, nonlast_name: "Brandon", last_name: "Crawford",
            batting_average: 0.246, on_base_percentage: 0.324,
            slugging_percentage: 0.389
        },
        shortstop: PositionPlayer {
            uniform_number: 4, nonlast_name: "Pablo", last_name: "Sandoval",
            batting_average: 0.279, on_base_percentage: 0.324,
            slugging_percentage: 0.415
        },
        leftfielder: PositionPlayer {
            uniform_number: 5, nonlast_name: "Mike", last_name: "Morse",
            batting_average: 0.279, on_base_percentage: 0.336,
            slugging_percentage: 0.475
        },
        centerfielder: PositionPlayer {
            uniform_number: 6, nonlast_name: "Angel", last_name: "Pagan",
            batting_average: 0.300, on_base_percentage: 0.342,
            slugging_percentage: 0.389
        },
        rightfielder: PositionPlayer {
            uniform_number: 7, nonlast_name: "Hunter", last_name: "Pence",
            batting_average: 0.277, on_base_percentage: 0.332,
            slugging_percentage: 0.445
        },

        lineup: [1, 2, 3, 4, 5, 6, 7, 8]
    };

    let home = Team {
        city: "Chicago",
        name: "Cubs",
        pitcher: Pitcher {
            uniform_number: 21, nonlast_name: "Mark", last_name: "Prior",
            walks_plus_hits_per_inning_pitched: 2.0, batting_average: 0.1
        },
        catcher: PositionPlayer {
            uniform_number: 8, nonlast_name: "Damian", last_name: "Miller",
            batting_average: 0.233, on_base_percentage: 0.310,
            slugging_percentage: 0.369
        },
        first_baseperson: PositionPlayer {
            uniform_number: 9, nonlast_name: "Eric", last_name: "Karros",
            batting_average: 0.286, on_base_percentage: 0.340,
            slugging_percentage: 0.446
        },
        second_baseperson: PositionPlayer {
            uniform_number: 10, nonlast_name: "Mark", last_name: "Grudzielanek",
            batting_average: 0.314, on_base_percentage: 0.366,
            slugging_percentage: 0.416
        },
        third_baseperson:  PositionPlayer {
            uniform_number: 11, nonlast_name: "Alex", last_name: "Gonzalez",
            batting_average: 0.228, on_base_percentage: 0.295,
            slugging_percentage: 0.409
        },
        shortstop:  PositionPlayer {
            uniform_number: 12, nonlast_name: "Aramis", last_name: "Ramirez",
            batting_average: 0.259, on_base_percentage: 0.314,
            slugging_percentage: 0.491
        },
        leftfielder:  PositionPlayer {
            uniform_number: 13, nonlast_name: "Moises", last_name: "Alou",
            batting_average: 0.280, on_base_percentage: 0.357,
            slugging_percentage: 0.462
        },
        centerfielder: PositionPlayer {
            uniform_number: 14, nonlast_name: "Corey", last_name: "Patterson",
            batting_average: 0.298, on_base_percentage: 0.329,
            slugging_percentage: 0.511
        },
        rightfielder:  PositionPlayer {
            uniform_number: 15, nonlast_name: "Taylor", last_name: "Swift",
            batting_average: 0.414, on_base_percentage: 0.500,
            slugging_percentage: 0.800
        },

        lineup: [1, 2, 3, 4, 5, 6, 7, 8]
    };

    let mut game_state = GameState {
        inning: 0,
        home_team: home,
        away_team: away,

        home_to_bat: 1,
        away_to_bat: 1,

        home_score: 0,
        away_score: 0,
        first_base: None,
        second_base: None,
        third_base: None,

        balls: 0,
        strikes: 0,
        outs: 0
    };

}
