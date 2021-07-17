class CreatePlayerAssignments < ActiveRecord::Migration[5.2]
  def change
    create_table :player_assignments do |t|
      t.belongs_to :player
      t.belongs_to :tournament

      t.timestamps
    end
  end
end
