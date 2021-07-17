class TournamentsController < ApplicationController
  def index
    @tournaments = Tournament.all
    render json: @tournaments
  end
  def show
    @tournament = Tournament.find(params[:id])
    render json: @tournament
  end
end
