use crate::GameState;
use crate::despawn_screen;
use bevy::prelude::*;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SplashTimer>()
            .add_systems(OnEnter(GameState::Splash), setup_splash_screen)
            .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
            .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
    }
}

#[derive(Component)]
struct OnSplashScreen;

#[derive(Resource)]
struct SplashTimer {
    timer: Timer,
}
impl Default for SplashTimer {
    fn default() -> Self {
        SplashTimer {
            timer: Timer::from_seconds(3.0, TimerMode::Once),
        }
    }
}

fn setup_splash_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    println!("Setup splash screen");
    let camera = Camera2d::default();

    commands.spawn((
        camera,
        Transform {
            translation: Vec3::new(640.0 / 2.0, 320.0 / 2.0, 0.0),
            ..default()
        },
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::camera::ScalingMode::AutoMax {
                max_width: (640.0),
                max_height: (320.0),
            },
            scale: 1.0,
            ..OrthographicProjection::default_2d()
        }),
        OnSplashScreen,
    ));

    let bg_layout = TextureAtlasLayout::from_grid(UVec2::new(1280, 640), 1, 1, None, None);
    let bg_layout_handle = texture_atlas_layouts.add(bg_layout);
    // Spawn the background image
    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/diamond_dash_splash.png"),
            texture_atlas: Some(TextureAtlas {
                layout: bg_layout_handle,
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(320.0, 160.0, -100.0).with_scale(Vec3::splat(0.5)),
        OnSplashScreen,
    ));

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            OnSplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Loading..."),
                TextFont {
                    font: Default::default(),
                    font_size: 40.0,
                    ..Default::default()
                },
                TextColor(Color::BLACK.into()),
            ));
        });
}

fn countdown(
    mut timer: ResMut<SplashTimer>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    timer.timer.tick(time.delta());
    if timer.timer.finished() {
        next_state.set(GameState::Menu);
    }
}
