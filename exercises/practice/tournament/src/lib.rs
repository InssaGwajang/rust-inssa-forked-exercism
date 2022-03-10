use self::MatchResult::*;
use std::collections::HashMap;

#[derive(Default)]
struct Team {
    name: String,
    matches: u8,
    wins: u8,
    draws: u8,
    losses: u8,
    points: u8,
}

impl Team {
    fn new(name: String) -> Self {
        Team {
            name: name,
            ..Default::default()
        }
    }

    fn record(&mut self, result: &MatchResult) {
        match result {
            Win => self.win(),
            Draw => self.draw(),
            Loss => self.lose(),
        }
    }
    fn win(&mut self) {
        self.matches += 1;
        self.wins += 1;
        self.points += 3;
    }
    fn draw(&mut self) {
        self.matches += 1;
        self.draws += 1;
        self.points += 1;
    }
    fn lose(&mut self) {
        self.matches += 1;
        self.losses += 1;
    }
}

const TALLY_HEADER: &str = "Team                           | MP |  W |  D |  L |  P";
impl From<&Team> for String {
    fn from(team: &Team) -> String {
        format!(
            "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
            team.name, team.matches, team.wins, team.draws, team.losses, team.points
        )
    }
}

enum MatchResult {
    Win,
    Draw,
    Loss,
}
impl From<&str> for MatchResult {
    fn from(s: &str) -> MatchResult {
        match s {
            "win" => Win,
            "draw" => Draw,
            "loss" => Loss,
            _ => panic!("invalid str for MatchResult"),
        }
    }
}
impl MatchResult {
    fn reverse(&self) -> Self {
        match self {
            Win => Loss,
            Loss => Win,
            Draw => Draw,
        }
    }
}

pub fn tally(match_results: &str) -> String {
    let mut teams: HashMap<String, Team> = HashMap::new();

    match_results.lines().for_each(|line| {
        let tokens: Vec<&str> = line.split(";").collect();
        let home = tokens[0];
        let away = tokens[1];
        let result = tokens[2];

        teams
            .entry(String::from(home))
            .or_insert(Team::new(String::from(home)))
            .record(&MatchResult::from(result));
        teams
            .entry(String::from(away))
            .or_insert(Team::new(String::from(away)))
            .record(&MatchResult::from(result).reverse());
    });

    let mut teams: Vec<&Team> = teams.values().collect();
    teams.sort_by(|a, b| b.points.cmp(&a.points).then_with(|| a.name.cmp(&b.name)));

    vec![String::from(TALLY_HEADER)]
        .into_iter()
        .chain(teams.into_iter().map(|t| String::from(t)))
        .collect::<Vec<String>>()
        .join("\n")
}

// use std::cmp::Ordering;

// #[derive(Debug, Clone, Copy, Default)]
// struct Team<'a> {
//     name: &'a str,
//     played: u32,
//     won: u32,
//     drawn: u32,
//     lost: u32,
//     point: u32,
// }

// pub fn tally(match_results: &str) -> String {
//     let mut teams = Vec::<Team>::new();
//     match_results.lines().for_each(|line| {
//         if let [mut team1, mut team2, mut result] =
//             line.split(";").collect::<Vec<&str>>().as_slice()
//         {
//             for name in [team1, team2] {
//                 if let None = teams.iter_mut().find(|&&mut team| team.name == name) {
//                     teams.push(Team {
//                         name: name,
//                         ..Default::default()
//                     })
//                 }
//             }
//             if result == "loss" {
//                 (team1, team2, result) = (team2, team1, "win")
//             }

//             if result == "win" {
//                 let mut team = teams
//                     .iter_mut()
//                     .find(|&&mut team| team.name == team1)
//                     .unwrap();

//                 team.played += 1;
//                 team.won += 1;
//                 team.point += 3;

//                 let mut team = teams
//                     .iter_mut()
//                     .find(|&&mut team| team.name == team2)
//                     .unwrap();

//                 team.played += 1;
//                 team.lost += 1;
//             } else if result == "draw" {
//                 for name in [team1, team2] {
//                     let mut team = teams
//                         .iter_mut()
//                         .find(|&&mut team| team.name == name)
//                         .unwrap();

//                     team.played += 1;
//                     team.drawn += 1;
//                     team.point += 1;
//                 }
//             }
//         }
//     });

//     teams.sort_by(|team1, team2| match team2.point.cmp(&team1.point) {
//         Ordering::Equal => team1.name.cmp(&team2.name),
//         ordering => ordering,
//     });

//     let mut result = String::from("Team                           | MP |  W |  D |  L |  P");
//     for team in teams.iter() {
//         result = format!(
//             "{}\n{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
//             result, team.name, team.played, team.won, team.drawn, team.lost, team.point
//         );
//     }

//     result
// }
