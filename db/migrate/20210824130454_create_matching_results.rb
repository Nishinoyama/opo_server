class CreateMatchingResults < ActiveRecord::Migration[5.2]
  def change
    create_table :matching_results do |t|
      t.belongs_to :matching, foreign_keys: true
      t.belongs_to :player, foreign_keys: true

      t.integer :win_count
      t.integer :draw_count
      t.integer :lose_count
    end
  end
end
