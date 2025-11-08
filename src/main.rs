use bevy::window::WindowResolution;
use bevy::{prelude::*, winit::WinitSettings};
use bevy_simple_text_input::TextInputPlugin;

use diamond_dash::GameState;
use diamond_dash::game::GamePlugin;
use diamond_dash::menu::MenuPlugin;
use diamond_dash::splash::SplashPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Diamond Dash".into(),
                        resolution: WindowResolution::new(640, 320),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        //.add_plugins(TextInputPlugin)
        .init_state::<GameState>()
        .insert_resource(WinitSettings::game())
        //.init_resource::<CharacterCreationData>()
        .add_plugins(SplashPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(GamePlugin)
        // add Game Over state
        .run();
}
