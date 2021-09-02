use crate::scoring::GameScore;
use MatchingStatus::*;

#[derive(Clone, Debug)]
/// A classification of Matching by status, such as normal, player dropped, opponent dropped, etc.
pub enum MatchingStatus {
    /// Matching without any problems is possessing Scoring data, and is the only possessing that.
    Normal(Matching, GameScore),
    /// Player had Dropped in the matching
    PlayerDropped(Matching),
    /// Opponent had Dropped in the matching
    OpponentDropped(Matching),
    /// Player had no matching due to there were odd numbers of players, then we treat that
    /// they had [`NoOpponent`] matching. The fields is a ID of the player.
    NoOpponent(usize),
    /// Matching with some fatal problems, which cannot be counted as Matching
    Invalid,
}

impl MatchingStatus {

    /// Returns points depending its [`MatchingStatus`].
    ///
    /// + if it is Normal, points are counted from its scoring.
    /// + if [`OpponentDropped`] or [`NoOpponent`], points are 3. (Default Win)
    /// + if [`PlayerDropped`], points are 0. (Default Lose)
    pub fn points(&self) -> usize {
        match self {
            Normal(_, scoring) => scoring.match_points(),
            OpponentDropped(_) | NoOpponent(_) => 3,
            PlayerDropped(_) | Invalid => 0,
        }
    }

    /// Returns game win percentage for calculating "Opponent Game Win Percentage(OGWP)"
    ///
    /// If it is NOT [`Normal`], returns 0.0.
    pub fn game_win_percentage(&self) -> f64 {
        match self {
            Normal(_, scoring) => scoring.win_percentage(),
            _ => 0.0
        }
    }

    /// Returns clone of self whose matching result were reversed.
    ///
    /// If it is [`NoOpponent`] or [`Invalid`], return clone of self.
    pub fn rev(&self) -> Self {
        if let Normal(matching, scoring) = self {
            Normal(matching.rev(), scoring.rev())
        } else if let PlayerDropped(matching) = self {
            OpponentDropped(matching.rev())
        } else if let OpponentDropped(matching) = self {
            PlayerDropped(matching.rev())
        } else {
            self.clone()
        }
    }

    /// Returns [`Matching`] data of this, which has IDs of player and opponent.
    ///
    /// # Panics
    ///
    /// Panics if it is [`NoOpponent`] or [`Invalid`]
    pub fn matching(&self) -> &Matching {
        match self {
            Normal(matching, _) | PlayerDropped(matching) | OpponentDropped(matching) => matching,
            _ => { panic!("No matching data") }
        }
    }

}

#[derive(Clone, Debug)]
/// Matched IDs
pub struct Matching {
    player_id: usize,
    opponent_id: usize,
}

impl Matching {

    /// Returns matched ID of player.
    pub fn player_id(&self) -> usize {
        self.player_id
    }

    /// Returns matched ID of opponent.
    pub fn opponent_id(&self) -> usize {
        self.opponent_id
    }

    /// Returns clone of Self whose IDs are swapped.
    pub fn rev(&self) -> Self {
        Self{ player_id: self.opponent_id, opponent_id: self.player_id }
    }

    /// Makes new [`Matching`].
    pub fn new(player_id: usize, opponent_id: usize) -> Self {
        Self{ player_id, opponent_id }
    }
}
