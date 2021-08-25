class TournamentsController < ApplicationController
  def index
    @tournaments = Tournament.all
  end
  def show
    @tournament = Tournament.find(params[:id])
    @players = @tournament.players
    @results = @tournament.matching_results
  end
end
