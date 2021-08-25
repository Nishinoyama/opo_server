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
  def matchings
    tournament = Tournament.find(params[:tournament_id])
    matchings = tournament.matchings
    matching_results = matchings.map { |m|
      {
        matching: m,
        matching_result: m.matching_result
      }
    }
    render json: matching_results
  end
end
