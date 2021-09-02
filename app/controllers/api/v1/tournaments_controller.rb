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
  def build
    tournament = Tournament.find(params[:tournament_id])
    time = Time.now.strftime("%Y%m%d%H%M%S%N")
    data_json = "#{time}.json"
    f = File.new(data_json, "w")
    f.write(tournament.players_sorted.to_json)
    render_data = `matching_builder_from_json #{data_json}`
    f.close
    File.unlink(data_json)
    render json: render_data
  end
end
