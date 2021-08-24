class CreateMatchings < ActiveRecord::Migration[5.2]
  def change
    create_table :matchings do |t|
      t.belongs_to :tournament, foreign_keys: true
      t.belongs_to :player, foreign_keys: true
      t.integer :matching_status
    end
  end
end
