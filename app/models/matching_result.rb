class MatchingResult < ApplicationRecord
  enum matching_status:  [ :finished, :running, :player_dropped, :opponent_dropped, :no_opponent, :invalid_result ]
  validates :win_count, :draw_count, :lose_count, presence: true, if: :finished?
  validates :tournament_id, uniqueness: {
    scope: [:player_id, :rounds],
    message: "Duplicated Matching"
  }

  def win?
    return true if opponent_dropped? || no_opponent?
    return win_count > lose_count if finished?
    false
  end

  def lose?
    return true if player_dropped?
    return lose_count > win_count if finished?
    false
  end

  def draw?
    return win_count == lose_count if finished?
    false
  end

  def points
    return 3 if win?
    return 1 if draw?
    0
  end

  def match_percentage
    points / 3.0
  end

  def game_percentage
    whole_count = win_count + draw_count + lose_count
    return 1.0 * (win_count * 3 + draw_count) / (3.0 * whole_count) if whole_count > 0
    0.0
  end

  def result_symbol
    if finished?
      return "O" if win?
      return "X" if lose?
      return "-" if draw?
    else
      matching_status
    end
  end

  def result_color
    if finished?
      return "#c0efd3" if win?
      return "#eec3c4" if lose?
      return "#f3e9cc" if draw?
    end
    "#cccccc"
  end

end
