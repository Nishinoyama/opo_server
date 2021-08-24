class TournamentsController < ApplicationController
  def index
    @tournaments = Tournament.all
  end
  def show
    @tournament = Tournament.find(params[:id])
    @players = @tournament.players
    @matchings = @tournament.matchings
  end
end
