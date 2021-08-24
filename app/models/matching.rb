class Matching < ApplicationRecord
  has_one :matching_result
  enum matching_status: [:normal, :player_dropped, :opponent_dropped, :no_opponent, :invalid_matching]
end
