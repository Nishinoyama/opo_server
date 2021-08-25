class CreateMatchingResults < ActiveRecord::Migration[5.2]
  def change
    create_table :matching_results do |t|
      t.belongs_to :tournament, foreign_key: true, null: false
      t.belongs_to :player, foreign_key: true, null: false
      # TODO: Find a method to define foreign keys without `t.bigint :opponent_id`
      t.bigint :opponent_id
      t.foreign_key :players, column: "opponent_id"

      t.integer :matching_status, null: false

      t.integer :win_count
      t.integer :draw_count
      t.integer :lose_count

      t.timestamps

    end
  end
end
