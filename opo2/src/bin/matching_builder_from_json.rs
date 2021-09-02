use std::env::args;
use std::fs::File;
use std::process::exit;
use std::io::BufReader;
use serde_json::{Value, json};
use opo::tournament::Tournament;
use opo::player::{Player, PlayerStatus};
use opo::matching::{MatchingStatus, Matching};
use opo::scoring::GameScore;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    if args().count() <= 1 {
        eprintln!("missing filename");
        exit(1);
    }
    let args:Vec<String> = args().collect();
    let file_name = args[1].clone();
    let file = File::open(file_name).unwrap_or_else(|e| {
        eprintln!("file doesn't existed");
        eprintln!("{:?}", e);
        exit(1);
    });
    let buf_reader = BufReader::new(file);
    let data_array: Vec<Value> = serde_json::from_reader(buf_reader)?;

    let mut local_id_to_original_id = Vec::new();
    for data in data_array.iter() {
        local_id_to_original_id.push(data["player"]["id"].as_u64().unwrap() as usize);
    }
    let local_id_to_original_id = local_id_to_original_id;
    let original_id_to_local_id = local_id_to_original_id.iter().enumerate().map(|(lid, gid)| {
        (*gid, lid)
    }).collect::<HashMap<usize, usize>>();

    let mut players = Vec::new();
    for data in data_array.iter() {
        let name = data["player"]["name"].to_string();
        let id = *original_id_to_local_id.get(&(data["player"]["id"].as_u64().unwrap() as usize)).unwrap();
        let mut player = Player::from_name(name, id);
        for result in data["results"].as_array().unwrap().iter() {
            let opponent_id = *original_id_to_local_id.get(
                &(result["opponent_id"].as_u64().unwrap_or(usize::MAX as u64) as usize)
            ).unwrap_or(&usize::MAX);
            let matching = Matching::new(id, opponent_id);
            let game_score = GameScore::new(
                result["win_count"].as_u64().unwrap_or(0) as usize,
                result["draw_count"].as_u64().unwrap_or(0) as usize,
                result["lose_count"].as_u64().unwrap_or(0) as usize,
            );
            player.add_matching( match result["matching_status"].as_str().unwrap() {
                "finished" => MatchingStatus::Normal(matching, game_score),
                "player_dropped" => MatchingStatus::PlayerDropped(matching),
                "opponent_dropped" => MatchingStatus::OpponentDropped(matching),
                "no_opponent" => MatchingStatus::NoOpponent(id),
                _ => MatchingStatus::Invalid,
            });
        }
        player.calculate_match_win_percentage();
        players.push(PlayerStatus::Normal(player));
    }
    let tournament = Tournament::copy_from_players_status(players);
    let next_matching =  tournament.matching_build();
    if let Ok(matching) = next_matching {
        let matching = matching.iter().enumerate().map(|(id, oid)|{
            let id = local_id_to_original_id[id];
            match oid {
                Some(iid) => {
                    let oid = local_id_to_original_id[*iid];
                    json!({ "player_id": id, "opponent_id": oid, "status": "matched" })
                }
                None => {
                    json!({ "player_id": id, "status": "no_opponent" })
                }
            }
        }).collect::<Vec<Value>>();
        println!("{}", json!(matching).to_string());
    } else {
        eprintln!("{:?}", next_matching.err().unwrap());
    }
    Ok(())
}