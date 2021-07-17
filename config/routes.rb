Rails.application.routes.draw do
  # For details on the DSL available within this file, see http://guides.rubyonrails.org/routing.html
  resources :players
  resources :tournaments do
    get "players", to: "players#index_in_tournament"
  end
end
