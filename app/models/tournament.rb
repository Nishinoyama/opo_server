class Tournament < ApplicationRecord
  has_many :player_assignments, dependent: :destroy
  has_many :players, through: :player_assignments
  has_many :matching_results, dependent: :destroy

  validates :name, presence: true

  def results_separated_by_player
    results_by_player = players.map { |p| [p.id, []] }.to_h
    matching_results.each { |m| results_by_player[m.player_id].push(m) }
    results_by_player
  end

  def players_win_percentages
    results_by_player = results_separated_by_player

    mwp_hash = results_by_player.map do |pid, results|
      [pid, any_percentage(results.select(&:finished?).map(&:match_percentage))]
    end.to_h
    gwp_hash = results_by_player.map do |pid, results|
      [pid, any_percentage(results.select(&:finished?).map(&:game_percentage))]
    end.to_h
    players_opponents_hash = results_by_player.map do |pid, results|
      [pid, results.select(&:finished?).map(&:opponent_id)]
    end.to_h

    results_by_player.map do |pid, results|
      [pid, {
        points: results.map(&:points).sum,
        mwp: mwp_hash[pid],
        omwp: any_percentage(players_opponents_hash[pid].map { |oid| [mwp_hash[oid], 1.0/3.0].max }),
        gwp: gwp_hash[pid],
        ogwp: any_percentage(players_opponents_hash[pid].map { |oid| gwp_hash[oid] }),
      }]
    end.to_h
  end

  def players_sorted
    scores = players_win_percentages
    players_results = results_separated_by_player
    scored_players = players.map do |player|
      {
        player: player,
        score: scores[player.id],
        results: players_results[player.id]
      }
    end
    scored_players.sort do |a, b|
      as, bs = a[:score], b[:score]
      (bs[:points] <=> as[:points]).nonzero? ||
        (bs[:omwp] <=> as[:omwp]).nonzero? ||
          (bs[:gwp] <=> as[:gwp]).nonzero? ||
            (bs[:ogwp] <=> as[:ogwp])
    end
  end

  private
  def any_percentage(points_array)
    return 0.0 if points_array.length == 0
    points_array.sum / points_array.length
  end

end
