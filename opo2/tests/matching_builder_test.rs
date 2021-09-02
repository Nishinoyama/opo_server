mod matching_builder_test {

    use opo::tournament::Tournament;
    use opo::matching::*;
    use opo::player::*;
    use opo::scoring::*;
    use std::collections::HashSet;

    #[test]
    fn test_matching_build() {
        let mut tv = vec![
            Tournament::from_players_name_dropped_list((0..501).into_iter().map(|i| {
                (format!("P{:04}", i), i % 100 == 1)
            }).collect())
        ];
        for i in 0..13 {
            let t = tv[i].clone();
            let l = t.matching_build().unwrap();
            let ml: Vec<MatchingStatus> = l.into_iter().enumerate().map(|(pid, o_oid)| {
                if let Some(oid) = o_oid {
                    (pid.min(oid), pid.max(oid))
                } else {
                    (pid, usize::max_value())
                }
            }).collect::<HashSet<(usize, usize)>>().into_iter().map(|(pid, oid)| {
                if oid == usize::max_value() {
                    MatchingStatus::NoOpponent(pid)
                } else if (oid * (i + 3)) % 127 == 50 {
                    MatchingStatus::OpponentDropped(
                        Matching::new(pid, oid),
                    )
                } else if (pid * (i * i + 32)) % 79 == 41 {
                    MatchingStatus::PlayerDropped(
                        Matching::new(pid, oid),
                    )
                } else {
                    MatchingStatus::Normal(
                        Matching::new(pid, oid),
                        GameScore::new(3, 2, 1),
                    )
                }
            }).collect();
            tv.push(t.aggregate_matches(ml));
            tv[i+1] = tv[i+1].drop_by_id_list(vec![(i*i+123)%500]);
            assert_eq!(
                format!("{:?}", tv[i+1]),
                format!("{:?}", Tournament::copy_from_players_status(tv[i+1].players().into_iter().cloned().collect())),
            );
            build_ranking_html(&tv[i+1], &format!("rank_{}.html", i+1));
        }
    }

    fn build_ranking_html(t: &Tournament, file_name: &String) {
        use std::fs::File;
        use std::io::prelude::*;

        let players = t.players();
        let mut ranked_players = players.clone();
        ranked_players.sort_by(| player, opponent| player.cmp(opponent));
        ranked_players.reverse();
        let mut file = File::create(file_name).unwrap();
        file.write_all(
            format!("<!DOCTYPE html> <html lang=\"en\"> <head> <meta charset=\"UTF-8\"> <title>{title}</title> <style> table, th, td {{ border: 1px solid black; }} </style> </head> <body>{tournament}</body> </html>",
                    title=file_name,
                    tournament=t.row(),
            ).into_bytes().as_slice()
        ).unwrap();
    }

    trait HTMLRow {
        fn row(&self) -> String;
    }

    impl HTMLRow for opo::tournament::Tournament {
        fn row(&self) -> String {
            let mut ranked_players = self.players();
            ranked_players.sort_by(|player, other| other.cmp(player));
            format!("<table> <thead> <tr> <th>ID</th> <th>Name</th> <th>Pts</th> <th>MWP</th> <th>OMWP</th> <th>GWP</th> <th>OGWP</th> {rounds} </tr> </thead> <tbody> {players} </tbody> </table>",
                    rounds=(1..=self.rounds()).into_iter().map(|i| format!("<th>R{}</th>", i)).collect::<String>(),
                    players=ranked_players.into_iter().map(|p| format!("{status}", status=p.row())).collect::<String>(),
            )
        }
    }

    impl HTMLRow for opo::player::PlayerStatus {
        fn row(&self) -> String {
            match self {
                PlayerStatus::Normal(player) =>
                    format!("<tr>{player}</tr>", player = player.row()),
                PlayerStatus::Dropped(player) =>
                    format!("<tr style=\"background-color: #d2d3e4\">{player}</tr>", player = player.row()),
                _ =>
                    format!(""),
            }
        }
    }

    impl HTMLRow for opo::player::Player {
        fn row(&self) -> String {
            format!("<td>{id}</td><td>{name}</td>{score}{matching}",
                    id=self.id(),
                    name=self.name().clone(),
                    score=self.score().row(),
                    matching=self.matching_list().iter().map(|ms| ms.row()).collect::<String>(),
            )
        }
    }

    impl HTMLRow for opo::player::PlayerScore {
        fn row(&self) -> String {
            format!("<td>{points}</td><td>{MWP:.3}</td><td>{OMWP:.3}</td><td>{GWP:.3}</td><td>{OGWP:.3}</td>",
                    points=self.points,
                    MWP=self.match_win_percentage,
                    OMWP=self.opponent_match_win_percentage,
                    GWP=self.game_win_percentage,
                    OGWP=self.opponent_game_win_percentage,
            )
        }
    }

    impl HTMLRow for opo::matching::MatchingStatus {
        fn row(&self) -> String {
            match self {
                MatchingStatus::Normal(matching, scoring) =>
                    format!("<td>{}:{}-{}-{}</td>", matching.opponent_id(), scoring.win_count, scoring.draw_count, scoring.lose_count ),
                MatchingStatus::PlayerDropped(matching) =>
                    format!("<td>{}:×</td>", matching.opponent_id()),
                MatchingStatus::OpponentDropped(matching) =>
                    format!("<td>{}:〇</td>", matching.opponent_id()),
                MatchingStatus::NoOpponent(_) =>
                    format!("<td>-:-</td>"),
                _ =>
                    format!("<td>?:?</td>"),
            }
        }
    }

}