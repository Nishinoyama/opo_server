class MatchingResult < ApplicationRecord
  enum matching_status:  [ :finished, :running, :player_dropped, :opponent_dropped, :no_opponent, :invalid_result ]
  validates :win_count, :draw_count, :lose_count, presence: true, if: :finished?

  def finished?
    matching_status == "finished"
  end
end
