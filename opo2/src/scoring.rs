#[derive(Clone, Debug)]
/// This struct has three fields, `win_count`, `draw_count`, `lose_count`.
pub struct GameScore {
    pub win_count: usize,
    pub draw_count: usize,
    pub lose_count: usize,
}

impl GameScore {
    /// Returns `true` if `win_count > lose_count`.
    pub fn is_winning(&self) -> bool {
        self.win_count > self.lose_count
    }

    /// Returns `true` if `win_count < lose_count`.
    pub fn is_losing(&self) -> bool {
        self.lose_count > self.win_count
    }

    /// Returns `true` if `win_count == lose_count`.
    pub fn is_drawing(&self) -> bool {
        !self.is_winning() && !self.is_losing()
    }

    /// Returns match points in normal matching.
    pub fn match_points(&self) -> usize {
        if self.is_winning() {
            3
        } else if self.is_drawing() {
            1
        } else {
            0
        }
    }

    #[inline]
    fn game_whole_points(&self) -> usize {
        (self.win_count + self.draw_count + self.lose_count) * 3
    }

    #[inline]
    fn game_points(&self) -> usize {
        self.win_count * 3 + self.draw_count
    }

    /// Returns game win percentage, for calculating "(Opponent) Game Win Percentage(GWP, OGWP)".
    ///
    /// If sum of counts of [`GameScore`] is 0, returns 0.0.
    pub fn win_percentage(&self) -> f64 {
        if self.game_whole_points() == 0 {
            0.0
        } else {
            self.game_points() as f64  / self.game_whole_points() as f64
        }
    }

    /// Returns clone of self whose result were reversed.
    pub fn rev(&self) -> Self {
        Self {
            win_count: self.lose_count,
            draw_count: self.draw_count,
            lose_count: self.win_count,
        }
    }

    /// Makes new [`GameScore`].
    pub fn new(win_count: usize, draw_count: usize, lose_count: usize) -> Self {
        Self{ win_count, draw_count, lose_count }
    }
}
