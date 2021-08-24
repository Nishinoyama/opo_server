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

10.times do |m|
  players = []
  100.times do |n|
    if (n * m) % 11 > 2
      PlayerAssignment.create!(tournament_id: m, player_id: n)
      players.push n
    end
  end
  nn = players.length
  nn.times do |n|
    3.times do |k|
      pid = players[n]
      oid = players[(n+k+2)%nn]
      wins = (n+k*2+3)%6
      loses = (n+k*7+2)%4
      draws = (n+k*8+1)%2
      matching = Matching.create(tournament_id: m, player_id: pid, matching_status: 0)
      MatchingResult.create!(matching_id: matching.id, player_id: oid, win_count: wins, draw_count: draws, lose_count: loses)
      matching = Matching.create(tournament_id: m, player_id: oid, matching_status: 0)
      MatchingResult.create!(matching_id: matching.id, player_id: pid, win_count: loses, draw_count: draws, lose_count: wins)
    end
  end
end
