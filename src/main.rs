use std::time::Duration;

use bevy::prelude::*;
use components::GameOverText;
use plugin::MyPlugin;
use setup::setup;
use systems::*;
use resources::*;

mod plugin;     //빌드 시스템 플러그인
mod constants;  //const 변수
mod setup;
mod components;
mod utils;
mod systems;
mod resources;

fn main(){
    App::new()
        .add_plugins(MyPlugin)
        .init_resource::<Game>()
        .add_systems(Startup, setup)
        .add_systems(Update, blink_space_bar_text.run_if(is_game_not_active))
        .add_systems(Update, move_background.run_if(is_game_active))
        .add_systems(Update, move_ground.run_if(is_game_active))
        .add_systems(Update, animate_fish.run_if(is_game_active))
        .add_systems(Update, start_game.run_if(is_game_not_active))
        .add_systems(Update, gravity.run_if(is_game_active))
        .add_systems(Update, jump.run_if(is_game_active))
        .add_systems(Update, pipes.run_if(is_game_active))
        .add_systems(Update, score.run_if(is_game_active))
        .add_systems(Update, render_score.run_if(is_game_active))
        .run();
} 

fn is_game_active(game: Res<Game>) -> bool{
    game.state == GameState::Active
}

fn is_game_not_active(game: Res<Game>) -> bool{
    game.state != GameState::Active
}