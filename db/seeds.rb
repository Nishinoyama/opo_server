# This file should contain all the record creation needed to seed the database with its default values.
# The data can then be loaded with the rails db:seed command (or created alongside the database with db:setup).
#
# Examples:
#
#   movies = Movie.create([{ name: 'Star Wars' }, { name: 'Lord of the Rings' }])
#   Character.create(name: 'Luke', movie: movies.first)

10.times do |n|
  Tournament.create!(name: "Seeded Tournament \##{n}")
end
100.times do |n|
  Player.create!(name: "Seeded Player \##{n}")
end

100.times do |n|
  10.times do |m|
    PlayerAssignment.create!(player_id: n, tournament_id: m) if (n * m) % 7 > 2
  end
end
