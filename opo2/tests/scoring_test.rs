mod scoring_test {

    use opo::scoring::GameScore;
    use opo::assert_ap;

    #[test]
    fn scoring_calculator() {
        let score = GameScore::new(3, 1, 2);
        assert!(score.is_winning());
        assert!(!score.is_drawing());
        assert!(!score.is_losing());
        assert_eq!(score.match_points(), 3);
        assert_ap!(score.win_percentage(), 0.556, 0.001); // = 10/18 = 0.556...
        let score = GameScore::new(1, 0, 4);
        assert!(!score.is_winning());
        assert!(!score.is_drawing());
        assert!(score.is_losing());
        assert_eq!(score.match_points(), 0);
        assert_ap!(score.win_percentage(), 0.200, 0.001); // = 3/15 = 0.200...
        let score = GameScore::new(0, 1, 0);
        assert!(!score.is_winning());
        assert!(score.is_drawing());
        assert!(!score.is_losing());
        assert_eq!(score.match_points(), 1);
        assert_ap!(score.win_percentage(), 0.333, 0.001); // = 3/15 = 0.333...
    }

    #[test]
    fn scoring_rev() {
        let score = GameScore::new(4, 2, 1);
        let rev_score = score.rev();
        assert_ap!(score.win_percentage(), 0.667, 0.001); // = 14/21 = 0.667...
        assert_ap!(rev_score.win_percentage(), 0.238, 0.001); // = 5/21 = 0.238...
    }

}
