# This file should contain all the record creation needed to seed the database with its default values.
# The data can then be loaded with the rails db:seed command (or created alongside the database with db:setup).
#
# Examples:
#
#   movies = Movie.create([{ name: 'Star Wars' }, { name: 'Lord of the Rings' }])
#   Character.create(name: 'Luke', movie: movies.first)

10.times do |n|
  Tournament.create!(name: "Seeded Tournament \##{n+1}")
end
100.times do |n|
  Player.create!(name: "Seeded Player \##{n+1}")
end

(1..10).each do |m|
  players = []
  (1..100).each do |n|
    if (n * m) % 11 > 2
      PlayerAssignment.create!(tournament_id: m, player_id: n)
      players.push n
    end
  end
  nn = players.length
  6.times do |k|
    rng = Random.new m + k + 2718
    shuffled_players = players.shuffle random: rng
    # pray for no occurrence of duplication!!
    nn.times do |n|
      if n & 2 == 0 && n == nn - 1
        pid = shuffled_players[n]
        rounds = k+1
        MatchingResult.create!(tournament_id: m, player_id: pid, rounds: rounds, matching_status: 4)
      elsif n % 2 == 0
        pid = shuffled_players[n]
        rounds = k+1
        oid = shuffled_players[n+1]
        pid, oid = oid, pid if pid > oid
        wins = (n+k*2+3)%6
        loses = (n+k*7+2)%4
        draws = (n+k*8+1)%2
        MatchingResult.create!(tournament_id: m, player_id: pid, rounds: rounds, matching_status: 0, opponent_id: oid, win_count: wins, draw_count: draws, lose_count: loses)
        MatchingResult.create!(tournament_id: m, player_id: oid, rounds: rounds, matching_status: 0, opponent_id: pid, win_count: loses, draw_count: draws, lose_count: wins)
      end
    end
  end
end
