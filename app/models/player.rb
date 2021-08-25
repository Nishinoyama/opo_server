class Player < ApplicationRecord
  has_many :player_assignments
  has_many :tournaments, through: :player_assignments
  has_many :matching_results
end
