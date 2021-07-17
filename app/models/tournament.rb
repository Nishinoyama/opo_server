class Tournament < ApplicationRecord
  has_many :player_assignments
  has_many :players, through: :player_assignments
end
