mod player_test {
    use opo::matching::{MatchingStatus, Matching};
    use opo::scoring::GameScore;
    use opo::player::{Player, PlayerStatus};
    use opo::assert_ap;
    use std::cmp::Ordering;
    use opo::matching::MatchingStatus::NoOpponent;

    #[test]
    fn test_add_matching() {
        // referenced: https://kirisamemagic.diarynote.jp/201401060210226433/
        let mut p = Player::from_name("あ😁し😁は😁ら".to_string(), 0);
        p.add_matching(MatchingStatus::Normal(Matching::new(0, 1), GameScore::new(2, 0, 0)));
        p.add_matching(MatchingStatus::NoOpponent(0));
        p.add_matching(MatchingStatus::PlayerDropped(Matching::new(0, 3)));
        p.add_matching(MatchingStatus::OpponentDropped(Matching::new(0, 4)));
        p.add_matching(MatchingStatus::Invalid);
        assert_eq!(p.matching_list().len(), 5);
        assert_eq!(p.matching_list()[0].points(), 3);
        assert_eq!(p.matching_list()[1].points(), 3);
        assert_eq!(p.matching_list()[2].points(), 0);
        assert_eq!(p.matching_list()[3].points(), 3);
        assert_eq!(p.matching_list()[4].points(), 0);
    }

    #[test]
    fn test_score_calculation() {
        // referenced: https://kirisamemagic.diarynote.jp/201401060210226433/
        let mut p = Player::from_name("あ😁し😁は😁ら".to_string(), 0);
        p.add_matching(MatchingStatus::Normal(Matching::new(0, 1), GameScore::new(2, 1, 0)));
        p.add_matching(MatchingStatus::PlayerDropped(Matching::new(0, 2)));
        p.add_matching(MatchingStatus::OpponentDropped(Matching::new(0, 3)));
        p.add_matching(MatchingStatus::NoOpponent(0));
        p.add_matching(MatchingStatus::Invalid);
        p.calculate_points();
        p.calculate_opponent_match_win_percentage(&vec![0.800,0.067,0.500,0.667,0.000,0.867]);
        p.calculate_game_win_percentage();
        p.calculate_opponent_game_win_percentage(&vec![0.800,0.067,0.500,0.667,0.000,0.867]);
        assert_eq!(p.score().points, 9);
        assert_ap!(p.score().opponent_match_win_percentage, 0.333, 0.001);
        assert_ap!(p.score().game_win_percentage, 0.777, 0.001);
        assert_ap!(p.score().opponent_game_win_percentage, 0.067, 0.001);
    }

    #[test]
    fn test_players_status_ord() {
        let p1 = PlayerStatus::Normal(Player::from_name("あ😁し😁は😁ら".to_string(), 0));
        let p2 = PlayerStatus::Dropped(Player::from_name("あ😁し😁は😁ら".to_string(), 1));
        let p3 = PlayerStatus::Dummy;
        assert_eq!(p1.cmp(&p2), Ordering::Greater);
        assert_eq!(p1.cmp(&p3), Ordering::Greater);
        assert_eq!(p3.cmp(&p2), Ordering::Less);
    }

    #[test]
    fn test_players_ord() {
        let mut ps = vec![
            Player::from_name("あ😁し😁は😁ら".to_string(), 0),
            Player::from_name("あ😁し😁は😁ら".to_string(), 1),
            Player::from_name("あ😁し😁は😁ら".to_string(), 2),
            Player::from_name("あ😁し😁は😁ら".to_string(), 3),
        ];
        ps[0].add_matching(MatchingStatus::Normal(Matching::new(0, 3), GameScore::new(0, 0, 3)));
        ps[0].calculate_points();
        ps[1].add_matching(MatchingStatus::Normal(Matching::new(1, 2), GameScore::new(1, 1, 1)));
        ps[1].calculate_points();
        ps[2].add_matching(MatchingStatus::Normal(Matching::new(2, 1), GameScore::new(1, 1,1)));
        ps[2].calculate_points();
        ps[3].add_matching(MatchingStatus::Normal(Matching::new(3, 0), GameScore::new(3, 0, 0)));
        ps[3].calculate_points();
        // pts = 0, 1, 1, 3
        assert_eq!(ps[0].cmp(&ps[3]), Ordering::Less);
        assert_eq!(ps[1].cmp(&ps[0]), Ordering::Greater);
        assert_eq!(ps[1].cmp(&ps[2]), Ordering::Equal);
    }

}