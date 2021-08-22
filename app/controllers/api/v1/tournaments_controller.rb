class Api::V1::TournamentsController < ApiController
  def index
    tournaments = Tournament.all
    render json: tournaments
  end
  def show
    tournament = Tournament.find(params[:id])
    render json: tournament
  end
  def players
    players = Tournament.find(params[:tournament_id]).players
    render json: players
  end
end
