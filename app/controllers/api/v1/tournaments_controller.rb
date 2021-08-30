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
  def matching_results
    matching_results = Tournament.find(params[:tournament_id]).matching_results
    render json: matching_results
  end
  def standing
    tournament = Tournament.find(params[:tournament_id])
    render json: tournament.players_sorted
  end
end
