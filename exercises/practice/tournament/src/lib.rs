use std::collections::HashMap;

use self::MatchResult::*;

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
            _ => panic!("invalid str:{} for MatchResult", s),
        }
    }
}

impl MatchResult {
    fn opposite(&self) -> Self {
        match self {
            Win => Loss,
            Loss => Win,
            Draw => Draw,
        }
    }
}

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
            name,
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

pub fn tally(match_results: &str) -> String {
    let mut teams: HashMap<String, Team> = HashMap::new();

    match_results.lines().for_each(|line| {
        let tokens: Vec<&str> = line.split(';').collect();
        let home = tokens[0];
        let away = tokens[1];
        let result = MatchResult::from(tokens[2]);

        teams
            .entry(String::from(home))
            .or_insert_with(|| Team::new(String::from(home)))
            .record(&result);
        teams
            .entry(String::from(away))
            .or_insert_with(|| Team::new(String::from(away)))
            .record(&result.opposite());
    });

    let mut teams: Vec<&Team> = teams.values().collect();
    teams.sort_by(|a, b| b.points.cmp(&a.points).then_with(|| a.name.cmp(&b.name)));

    vec![String::from(TALLY_HEADER)]
        .into_iter()
        .chain(teams.into_iter().map(String::from))
        .collect::<Vec<String>>()
        .join("\n")
}
