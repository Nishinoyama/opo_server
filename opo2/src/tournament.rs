use crate::player::{PlayerStatus, Player};
use crate::matching::MatchingStatus;
use crate::player::PlayerStatus::*;

#[derive(Clone, Debug)]
pub struct Tournament{
    player_number: usize,
    matched_rounds: usize,
    players: Vec<PlayerStatus>,
}

impl Tournament {

    /// Creates new [`Tournament`] from a player's name list.
    pub fn from_players_name_list(name_list: Vec<String>) -> Self {
        Tournament {
            player_number: name_list.len(),
            matched_rounds: 0,
            players: name_list.into_iter().enumerate().map(|(id, name)| {
                PlayerStatus::Normal(Player::from_name(name, id))
            }).collect(),
        }
    }

    /// Creates new [`Tournament`] from a player's name and dropped (or not) list.
    pub fn from_players_name_dropped_list(name_list: Vec<(String, bool)>) -> Self {
        Tournament {
            player_number: name_list.len(),
            matched_rounds: 0,
            players: name_list.into_iter().enumerate().map(|(id, (name, dropped))| {
                if dropped {
                    Dropped(Player::from_name(name, id))
                } else {
                    Normal(Player::from_name(name, id))
                }
            }).collect(),
        }
    }

    fn destruct_players(&self) -> Vec<&Player> {
        use PlayerStatus::*;
        self.players.iter().filter_map(|status| {
            match status {
                Normal(player) | Dropped(player) => Some(player),
                Dummy => None,
            }
        }).collect()
    }

    fn destruct_mut_players(&mut self) -> Vec<&mut Player> {
        use PlayerStatus::*;
        self.players.iter_mut().filter_map(|status| {
            match status {
                Normal(player) | Dropped(player) => Some(player),
                Dummy => None,
            }
        }).collect()
    }

    /// Resumes [`Tournament`] from a list of [`PlayerStatus`].
    pub fn copy_from_players_status(players: Vec<PlayerStatus>) -> Self {
        Tournament {
            player_number: players.len(),
            matched_rounds: players[0].destruct_player().matching_list().len(),
            players,
        }.aggregate_points()
    }

    fn calculate_points(&self) -> Self {
        let mut tournament = self.clone();
        for player in tournament.destruct_mut_players() {
            player.calculate_points();
        }
        tournament
    }

    fn calculate_match_win_percentages(&self) -> Self {
        let mut tournament = self.clone();
        for player in tournament.destruct_mut_players() {
            player.calculate_match_win_percentage();
        }
        tournament
    }

    fn calculate_opponent_match_win_percentages(&self) -> Self {
        let mut tournament = self.clone();
        let players_mwp = tournament.destruct_players().iter()
            .map(|player| player.score().match_win_percentage())
            .collect();
        for player in tournament.destruct_mut_players() {
            player.calculate_opponent_match_win_percentage(&players_mwp);
        }
        tournament
    }

    fn calculate_game_win_percentages(&self) -> Self {
        let mut tournament = self.clone();
        for player in tournament.destruct_mut_players() {
            player.calculate_game_win_percentage();
        }
        tournament
    }

    fn calculate_opponent_game_win_percentages(&self) -> Self {
        let mut tournament = self.clone();
        let players_gwp = tournament.destruct_players().iter()
            .map(|player| player.score().game_win_percentage())
            .collect::<Vec<f64>>();
        for player in tournament.destruct_mut_players() {
            player.calculate_opponent_game_win_percentage(&players_gwp);
        }
        tournament
    }

    fn aggregate_points(&self) -> Self {
        self
            .calculate_points()
            .calculate_match_win_percentages()
            .calculate_opponent_match_win_percentages()
            .calculate_game_win_percentages()
            .calculate_opponent_game_win_percentages()
    }

    /// Aggregates given [`MatchingStatus`] list, makes players calculate their score
    /// and solves their OMWP, OGWP. Then, creates [`Tournament`] of the next round and returns it.
    pub fn aggregate_matches(&self, matches: Vec<MatchingStatus>) -> Self {
        use MatchingStatus::*;
        let mut tournament = self.clone();
        let mut matched_status = vec![Invalid; tournament.player_number];
        matches.iter().for_each(|matching_status| {
            match matching_status {
                Normal(matching,_) | PlayerDropped(matching) | OpponentDropped(matching) => {
                    let (player_id, opponent_id) = (matching.player_id(), matching.opponent_id());
                    if !matches!(matched_status[player_id], Invalid) {
                        panic!("ID: {} had matched, some matching are duplicated.", player_id);
                    }
                    if !matches!(matched_status[opponent_id], Invalid) {
                        panic!("ID: {} had matched, some matching are duplicated.", opponent_id);
                    }
                    matched_status[player_id] = matching_status.clone();
                    matched_status[opponent_id] = matching_status.rev();
                }
                NoOpponent(player_id) => {
                    let player_id = *player_id;
                    if !matches!(tournament.players[player_id], PlayerStatus::Dropped(_)) {
                        if !matches!(matched_status[player_id], Invalid) {
                            panic!("ID: {} had matched, but matching list contains its some matching.", player_id);
                        }
                        matched_status[player_id] = NoOpponent(player_id);
                    }
                }
                _ => {}
            }
        });
        let players = tournament.destruct_mut_players();
        players.into_iter().enumerate().for_each(|(id, player)|{
            player.add_matching(matched_status[id].clone());
        });
        tournament.matched_rounds += 1;
        tournament.aggregate_points()

    }

    /// Returns `Vec<Option<usize>>` whose `Some(m)` n-th element number has n-th id player's opponent m
    /// if element is `None`, no-opponent or player is dropped (No matching)
    pub fn matching_build(&self) -> Result<Vec<Option<usize>>, String> {
        crate::matching_builder::matching_build(&self.players)
    }

    /// Returns its players' [`PlayerStatus`] pointers [`Vec`].
    pub fn players(&self) -> Vec<&PlayerStatus> {
        self.players.iter().collect()
    }

    /// Returns its number of matched rounds.
    pub fn rounds(&self) -> usize {
        self.matched_rounds
    }

    /// Drop its players by given ids list.
    pub fn drop_by_id_list(&self, id_list: Vec<usize>) -> Self {
        let mut t = self.clone();
        for i in id_list {
            let player_status = &t.players[i];
            t.players[i] = match player_status {
                Normal(player) => Dropped(player.clone()),
                Dropped(player ) => Dropped(player.clone()),
                Dummy => Dummy,
            }
        }
        t
    }

    // pub fn player_number(&self) -> usize {
    //     self.players.len()
    // }

}
