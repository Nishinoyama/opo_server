# This file is auto-generated from the current state of the database. Instead
# of editing this file, please use the migrations feature of Active Record to
# incrementally modify your database, and then regenerate this schema definition.
#
# Note that this schema.rb definition is the authoritative source for your
# database schema. If you need to create the application database on another
# system, you should be using db:schema:load, not running all the migrations
# from scratch. The latter is a flawed and unsustainable approach (the more migrations
# you'll amass, the slower it'll run and the greater likelihood for issues).
#
# It's strongly recommended that you check this file into your version control system.

ActiveRecord::Schema.define(version: 2021_08_24_130454) do

  # These are extensions that must be enabled in order to support this database
  enable_extension "plpgsql"

  create_table "matching_results", force: :cascade do |t|
    t.bigint "matching_id"
    t.bigint "player_id"
    t.integer "win_count"
    t.integer "draw_count"
    t.integer "lose_count"
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["matching_id"], name: "index_matching_results_on_matching_id"
    t.index ["player_id"], name: "index_matching_results_on_player_id"
  end

  create_table "matchings", force: :cascade do |t|
    t.bigint "tournament_id"
    t.bigint "player_id"
    t.integer "matching_status"
    t.index ["player_id"], name: "index_matchings_on_player_id"
    t.index ["tournament_id"], name: "index_matchings_on_tournament_id"
  end

  create_table "player_assignments", force: :cascade do |t|
    t.bigint "player_id"
    t.bigint "tournament_id"
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["player_id"], name: "index_player_assignments_on_player_id"
    t.index ["tournament_id"], name: "index_player_assignments_on_tournament_id"
  end

  create_table "players", force: :cascade do |t|
    t.string "name"
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
  end

  create_table "tournaments", force: :cascade do |t|
    t.string "name"
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
  end

end
