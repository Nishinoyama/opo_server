class Player < ApplicationRecord
  has_many :player_assignments
  has_many :tournaments, through: :player_assignments
  has_many :matching_results

  def results(tournament_id)
    matching_results.select{|m| m.tournament_id == tournament_id }
  end

  def points(tournament_id)
    results(tournament_id).map(&:points).sum
  end

  def opponents(tournament_id)
    results(tournament_id).map(&:opponent_id)
  end

  def match_win_percentage(tournament_id)
    any_percentage(results(tournament_id).select(&:finished?).map(&:match_percentage))
  end

  def opponent_match_win_percentage(tournament_id, mwp_hash)
    opponents_ids = opponents tournament_id
    any_percentage(mwp_hash.select{|id, _| opponents_ids.include?(id) }.map{|_, mwp| [mwp, 1.0/3].max})
  end

  def game_win_percentage(tournament_id)
    any_percentage(results(tournament_id).select(&:finished?).map(&:game_percentage))
  end

  def opponent_game_win_percentage(tournament_id, gwp_hash)
    opponents_ids = opponents tournament_id
    any_percentage(gwp_hash.select{|id, _| opponents_ids.include?(id) }.map{|_, gwp| gwp})
  end

  def scoring(tournament_id, mwp_hash, gwp_hash)
    {
      mwp: match_win_percentage(tournament_id),
      omwp: opponent_match_win_percentage(tournament_id, mwp_hash),
      gwp: game_win_percentage(tournament_id),
      ogwp: opponent_game_win_percentage(tournament_id, gwp_hash),
    }
  end

  private
  def any_percentage(points_array)
    return 0.0 if points_array.length == 0
    points_array.sum / points_array.length
  end

end
