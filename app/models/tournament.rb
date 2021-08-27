class Tournament < ApplicationRecord
  has_many :player_assignments
  has_many :players, through: :player_assignments
  has_many :matching_results

  def mwp_hash
    players.map{|p| [p.id, p.match_win_percentage(id)] }.to_h
  end

  def gwp_hash
    players.map{|p| [p.id, p.game_win_percentage(id)] }.to_h
  end

  def players_sorted(players)
    players.sort do |a, b|
      (b.points(id) <=> a.points(id)).nonzero? ||
        (b.opponent_match_win_percentage(id, mwp_hash) <=> a.opponent_match_win_percentage(id, mwp_hash) )
    end
  end

end
