pub mod game;
pub mod menu;
pub mod splash;
pub mod gameover;

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    InGame,
    GameOver,
    Leaderboard,
    Credits,
    Settings,
}

// Generic despawn system for cleanup
pub fn despawn_screen<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    //println!("Despawn screen: {:?}", &query);
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
