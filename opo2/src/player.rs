use crate::matching::{MatchingStatus, Matching};
use std::cmp::Ordering;
use crate::scoring::GameScore;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
/// A classifying by [`Player`] is Normal or Dropped or Dummy.
pub enum PlayerStatus{
    /// Might be used in matching algorithm with odd number of the players.
    Dummy,
    /// Player who dropped out of the tournament. Player won't be matched any more.
    Dropped(Player),
    /// Player with no problems.
    Normal(Player),
}

impl PlayerStatus {
    /// Returns [`Player`] belonging to it.
    ///
    /// Panic
    /// --
    ///
    /// Panics if it is [`PlayerStatus::Dummy`]
    pub fn destruct_player(&self) -> &Player {
        match self {
            Self::Normal(player) | Self::Dropped(player) => player,
            _ => panic!("No Player Data!"),
        }
    }
}

#[derive(Clone, Debug)]
/// A player that have their name, score in Tournament, and status list they matched.
pub struct Player {
    id: usize,
    name: String,
    score: PlayerScore,
    matching_list: Vec<MatchingStatus>,
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Player {}
impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
    }
}

impl Player {

    /// Makes new [`Player`] by their name.
    pub fn from_name(name: String, id: usize) -> Self {
        Player {
            id,
            name: name.clone(),
            score: PlayerScore::default(),
            matching_list: vec![],
        }
    }

    /// Returns number of [MatchingStatus::Normal] matches the player matched.
    pub fn matched_round_number(&self) -> usize {
        use MatchingStatus::*;
        self.matching_list.iter()
            .filter(|matching| matches!(matching, Normal(_, _)))
            .count()
    }

    /// Returns their [`PlayerScore`] they earned.
    pub fn score(&self) -> &PlayerScore {
        &self.score
    }

    /// Returns their [`Vec`] of [`MatchingStatus`] they matched.
    pub fn matching_list(&self) -> &Vec<MatchingStatus> {
        &self.matching_list
    }

    fn destruct_valid_matching_list(&self) -> Vec<(&Matching, &GameScore)> {
        use MatchingStatus::*;
        self.matching_list.iter().filter_map(|matching_status|{
            if let Normal(matching, scoring) = matching_status {
                Some((matching, scoring))
            } else {
                None
            }
        }).collect()
    }

    fn destruct_matched_id_list(&self) -> Vec<Option<usize>> {
        use MatchingStatus::*;
        self.matching_list.iter()
            .filter_map(|matching_status| {
                match matching_status {
                    Normal(matching, _) | PlayerDropped(matching) | OpponentDropped(matching) => {
                        Some(Some(matching.opponent_id()))
                    }
                    NoOpponent(_) => Some(None),
                    _ => None,
                }
            }).collect()
    }

    fn any_percentage(opponents_wp: Vec<f64>) -> f64 {
        let count = opponents_wp.len();
        let sum = opponents_wp.into_iter().sum::<f64>();
        return if count > 0 {
            sum / count as f64
        } else {
            0.0
        }
    }

    fn matching_list_to_percentage_list<PercentageFn>(
        matching_list: &Vec<(&Matching, &GameScore)>,
        mut percentage_fn: PercentageFn,
    ) -> Vec<f64>
        where PercentageFn: FnMut(&Matching) -> f64
    {
        matching_list.into_iter()
            .map(|(matching, _)| percentage_fn(matching))
            .collect()
    }

    /// Calculates their points destructively.
    pub fn calculate_points(&mut self) {
        self.score.points = self.matching_list.iter()
            .map(|matching| matching.points())
            .sum();
    }

    /// Calculates their MWP destructively.
    pub fn calculate_match_win_percentage(&mut self) {
        self.score.match_win_percentage = Self::any_percentage(
            self.destruct_valid_matching_list().into_iter()
                .map(|(_, scoring)| (scoring.match_points() as f64 / 3.0) ).collect()
        )
    }

    /// Calculates their OMWP destructively by the other players' MWP.
    pub fn calculate_opponent_match_win_percentage(&mut self, players_mwp: &Vec<f64>) {
        let omwp_list: Vec<f64> = Self::matching_list_to_percentage_list(
            &self.destruct_valid_matching_list(),
            |matching| players_mwp.get(matching.opponent_id()).unwrap().max(1.0 / 3.0)
        );
        self.score.opponent_match_win_percentage = Self::any_percentage(omwp_list);
    }

    /// Calculates their GWP destructively.
    pub fn calculate_game_win_percentage(&mut self) {
        let gwp_list = self.destruct_valid_matching_list().iter().map(|(_, scoring)|{
            scoring.win_percentage()
        }).collect();
        self.score.game_win_percentage = Self::any_percentage(gwp_list);
    }

    /// Calculates their OGWP destructively by the other players' GWP.
    pub fn calculate_opponent_game_win_percentage(&mut self, players_gwp: &Vec<f64>) {
        let ogwp_list: Vec<f64> = Self::matching_list_to_percentage_list(
            &self.destruct_valid_matching_list(),
            |matching| *players_gwp.get(matching.opponent_id()).unwrap()
        );
        self.score.opponent_game_win_percentage = Self::any_percentage(ogwp_list)
    }

    /// Returns whether the player has matched with opponent with given id if the id is [`Some`],
    ///
    /// Otherwise, returns whether the player has matched [`MatchingStatus::NoOpponent`] if the id is [`None`].
    ///
    /// # Example
    /// ```
    /// use opo::player::Player;
    /// use opo::matching::{MatchingStatus, Matching};
    /// use opo::scoring::GameScore;
    ///
    /// let mut player = Player::from_name(String::from("あ😁し😊は😂ら"), 0);
    /// player.add_matching(MatchingStatus::Normal(Matching::new(0, 2), GameScore::new(3, 2, 1)));
    /// player.add_matching(MatchingStatus::NoOpponent(0));
    ///
    /// assert!(player.had_matched_id(Some(2)));
    /// assert!(!player.had_matched_id(Some(1)));
    /// assert!(player.had_matched_id(None));  // for no opponent matching
    /// ```
    ///
    pub fn had_matched_id(&self, search_id: Option<usize>) -> bool {
        self.destruct_matched_id_list().contains(&search_id)
    }

    /// Adds new [`MatchingStatus`] their matching list destructively.
    pub fn add_matching(&mut self, matching: MatchingStatus) {
        self.matching_list.push(matching);
    }

    /// Returns its id.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Returns its name.
    pub fn name(&self) -> String {
        self.name.clone()
    }

}

/// The [`Player`] score such as `points`, `MWP`, `OMWP`, `GWP`, `OGWP`.
///
/// Note that this struct has no implementations for calculating score and [`Player`] managed it.
/// The reason why it does is this struct does not have scores on matching and that it is burdensome
/// for the programmer to make function to calculate the score with serving such as their MWP and GWP.
///
/// # Ordering by
///
/// 1. their `points` is greater
/// 2. their `OMWP` is greater
/// 3. their `GWP` is greater
/// 4. their `OGWP` is greater
#[derive(Clone, PartialEq, Default, Debug)]
pub struct PlayerScore {
    pub points: usize,
    pub match_win_percentage: f64,
    pub opponent_match_win_percentage: f64,
    pub game_win_percentage: f64,
    pub opponent_game_win_percentage: f64,
}

impl Eq for PlayerScore {}

impl Ord for PlayerScore {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.points.ne(&other.points) {
            self.points.cmp(&other.points)
        } else if self.opponent_match_win_percentage.ne(&other.opponent_match_win_percentage) {
            self.opponent_match_win_percentage.partial_cmp(&other.opponent_match_win_percentage).unwrap()
        } else if self.game_win_percentage.ne(&other.game_win_percentage) {
            self.game_win_percentage.partial_cmp(&other.game_win_percentage).unwrap()
        } else if self.opponent_game_win_percentage.ne(&other.opponent_game_win_percentage) {
            self.opponent_game_win_percentage.partial_cmp(&other.opponent_game_win_percentage).unwrap()
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for PlayerScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PlayerScore {
    /// Returns their MWP for the other OMWP.
    pub fn match_win_percentage(&self) -> f64 {
        self.match_win_percentage
    }

    /// Returns their GWP for the other OGWP.
    pub fn game_win_percentage(&self) -> f64 {
        self.game_win_percentage
    }
}
