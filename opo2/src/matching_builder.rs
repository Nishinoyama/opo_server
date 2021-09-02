use crate::player::{PlayerStatus, Player};
use std::collections::BinaryHeap;

pub fn matching_build(players: &Vec<PlayerStatus>) -> Result<Vec<Option<usize>>, String> {

    let mut matchable_players = filter_sorted_matchable_players(players);
    let dummy_player = Player::from_name("Dummy".to_string(), usize::max_value());
    if matchable_players.len() % 2 == 1 {
        matchable_players.push((None, &dummy_player))
    }
    let matchable_players = matchable_players;
    let matchable_number: usize = matchable_players.len();

    for ext in 6..26 {
        let mut dp = vec![vec![i64::max_value(); 1 << ext]; matchable_number + 1];
        let mut rb = vec![vec![(-1, false); 1 << ext]; matchable_number + 1];
        dp[0][0] = 0;
        let mut pq = BinaryHeap::new();
        pq.push((0, 0, 0));
        while !pq.is_empty() {
            let (di, ni, bi) = pq.pop().unwrap();
            let di = -di;
            if dp[ni][bi] > di {
                continue;
            }
            let nni = ni + 1;
            if bi & 1 == 1 {
                if dp[ni][bi] < dp[nni][bi >> 1] {
                    dp[nni][bi >> 1] = dp[ni][bi];
                    rb[nni][bi >> 1] = (bi as i32, false);
                    pq.push((-dp[ni][bi], nni, bi>>1));
                }
                continue;
            }
            if ni == matchable_number {
                break;
            }
            for pi in 0..ext {
                let ppi = ni + pi + 1;
                if ppi >= matchable_number {
                    break;
                }
                if ((bi >> 1) & (1 << pi)) != 0 {
                    continue;
                }
                let (_player_id, player) = matchable_players[ni];
                let (opponent_id, opponent) = matchable_players[ppi];
                let bbi = (bi >> 1) | (1 << pi);
                let cost = if opponent_id.is_some() {
                    ((player.score().points as i64).pow(2) - (opponent.score().points as i64).pow(2)).pow(2)
                } else {
                    (player.score().points.pow(4)) as i64
                };
                if !player.had_matched_id(opponent_id) {
                    if dp[ni][bi] + cost < dp[nni][bbi] {
                        dp[nni][bbi] = dp[ni][bi] + cost;
                        rb[nni][bbi] = (bi as i32, true);
                        pq.push((-dp[nni][bbi], nni, bbi));
                    }
                }
            }
        }

        // rollback
        let mut matching_list = vec![None; players.len()];
        let mut rbn = matchable_number;
        let mut rbb: usize = 0;
        let matching_success = loop {
            let tmp_rbb = rb[rbn][rbb].0;
            if tmp_rbb < 0 {
                break false;
            }
            if rb[rbn][rbb].1 {
                let transition = (rbb<<1) - tmp_rbb as usize;
                let lid = matchable_players[rbn - 1].0;
                let rid = matchable_players[rbn - 1 + transition.trailing_zeros() as usize].0;
                if lid.is_some() && rid.is_some() {
                    matching_list[lid.unwrap()] = rid;
                    matching_list[rid.unwrap()] = lid;
                }
            }
            rbn -= 1;
            rbb = tmp_rbb as usize;
            if rbn == 0 && rbb == 0 {
                break true;
            }
        };

        if matching_success {
            return Ok(matching_list);
        }

    }

    Err("No satisfying matching!".to_string())

}

fn filter_sorted_matchable_players(players: &Vec<PlayerStatus>) -> Vec<(Option<usize>, &Player)> {
    let mut players = players.into_iter()
        .filter_map(|player_status| {
            match player_status {
                PlayerStatus::Normal(player) => Some((Some(player.id()), player)),
                _ => None,
            }
        })
        .collect::<Vec<(Option<usize>, &Player)>>();
    players.sort_by(|(_, player), (_, other)| other.cmp(player));
    let players = players;
    players
}
