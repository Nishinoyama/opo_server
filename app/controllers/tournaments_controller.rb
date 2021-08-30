class TournamentsController < ApplicationController
  def index
    @tournaments = Tournament.all
  end

  def show
    @tournament = Tournament.find(params[:id])
    @players = @tournament.players
    @results = @tournament.matching_results
  end

  def new
    @tournament = Tournament.new
  end

  def create
    @tournament = Tournament.new(tournament_params)

    if @tournament.save
      redirect_to @tournament
    else
      render :new
    end
  end

  def edit
    @tournament = Tournament.find(params[:id])
  end

  def update
    @tournament = Tournament.find(params[:id])

    if @tournament.update(tournament_params)
      redirect_to @tournament
    else
      render :edit
    end
  end

  def destroy
    @tournament = Tournament.find(params[:id])
    @tournament.destroy

    redirect_to tournaments_path
  end

  private
  def tournament_params
    params.require(:tournament).permit(:name)
  end
end
