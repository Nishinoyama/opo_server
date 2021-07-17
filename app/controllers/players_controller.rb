class PlayersController < ApplicationController
  def index
    @players = Player.all
    render json: @players
  end
  def show
    @player = Player.find(params[:id])
    render json: @player
  end
  def index_in_tournament
    @tournament = Tournament.find(params[:tournament_id])
    @players = @tournament.players
    render json: @players
  end
end
