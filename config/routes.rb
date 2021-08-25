Rails.application.routes.draw do
  # For details on the DSL available within this file, see http://guides.rubyonrails.org/routing.html
  resources :players
  resources :tournaments

  namespace :api do
    namespace :v1 do
      resources :tournaments, only: [:index, :show] do
        get 'players', 'matchings'
      end
      resources :players, only: [:index, :show]
    end
  end
end
