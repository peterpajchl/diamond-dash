mod enemies;
mod hero;
mod score;

use crate::GameState;
use crate::despawn_screen;
use bevy::prelude::*;
use enemies::{Enemy, enemies_movement, setup_enemies};
use hero::{
    AnimationData, AnimationIndices, Player, PlayerAnimationData, animate_sprite, player_movement,
    setup_hero, update_player_animation,
};
use rand::Rng;
use score::{Score, setup_score_ui, update_score_ui};

pub struct GamePlugin;

#[derive(Component)]
struct OnGameScreen;

#[derive(Component)]
struct Diamond;

#[derive(Component)]
struct Background;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Score>()
            .add_systems(
                OnEnter(GameState::InGame),
                (
                    setup_game,
                    setup_background,
                    setup_hero.after(setup_game),
                    setup_enemies.after(setup_game),
                    setup_diamonds.after(setup_game),
                    setup_score_ui,
                ),
            )
            .add_systems(
                Update,
                (
                    player_movement,
                    animate_sprite,
                    update_player_animation,
                    update_score_ui,
                    collision_detection,
                    collision_detection_diamonds,
                    enemies_movement,
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(GameState::InGame), despawn_screen::<OnGameScreen>);
    }
}

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    println!("Setup game");
    // Load textures for all animations
    let texture_idle = asset_server.load::<Image>("sprites/characters/hero/idle/idle.png");
    let texture_walk = asset_server.load::<Image>("sprites/characters/hero/walk/walk.png");
    let texture_run = asset_server.load::<Image>("sprites/characters/hero/dash/dash.png");
    let texture_jump = asset_server.load::<Image>("sprites/characters/hero/jump/normal/jump.png");

    // Create texture atlases for each animation
    let idle_atlas_layout = TextureAtlasLayout::from_grid(UVec2::new(48, 64), 8, 6, None, None);
    let walk_atlas_layout = TextureAtlasLayout::from_grid(UVec2::new(48, 64), 8, 6, None, None);
    let run_atlas_layout = TextureAtlasLayout::from_grid(UVec2::new(48, 64), 8, 6, None, None);
    let jump_atlas_layout = TextureAtlasLayout::from_grid(UVec2::new(48, 64), 8, 6, None, None);

    let idle_atlas_handle = texture_atlas_layouts.add(idle_atlas_layout);
    let walk_atlas_handle = texture_atlas_layouts.add(walk_atlas_layout);
    let run_atlas_handle = texture_atlas_layouts.add(run_atlas_layout);
    let jump_atlas_handle = texture_atlas_layouts.add(jump_atlas_layout);

    let idle_frames = AnimationIndices::new(0, 7);
    let walk_frames = AnimationIndices::new(0, 7);
    let jump_frames = AnimationIndices::new(0, 7);
    let run_frames = AnimationIndices::new(0, 7);

    commands.insert_resource(PlayerAnimationData::new(
        AnimationData {
            texture_atlas: idle_atlas_handle,
            texture: texture_idle,
            frames: idle_frames,
        },
        AnimationData {
            texture_atlas: walk_atlas_handle,
            texture: texture_walk,
            frames: walk_frames,
        },
        AnimationData {
            texture_atlas: jump_atlas_handle,
            texture: texture_jump,
            frames: jump_frames,
        },
        AnimationData {
            texture_atlas: run_atlas_handle,
            texture: texture_run,
            frames: run_frames,
        },
    ));

    // Camera
    let camera = Camera2d::default();

    commands.spawn((
        camera,
        Transform {
            translation: Vec3::new(640.0 / 2.0, 320.0 / 2.0, 0.0),
            ..default()
        },
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::camera::ScalingMode::AutoMax {
                max_width: 640.0,
                max_height: 320.0,
            },
            scale: 1.0,
            ..OrthographicProjection::default_2d()
        }),
        OnGameScreen
    ));
}

fn setup_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    println!("Setup background");
    let bg_layout = TextureAtlasLayout::from_grid(UVec2::new(1280, 640), 1, 1, None, None);
    let bg_layout_handle = texture_atlas_layouts.add(bg_layout);
    // Spawn the background image
    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/diamond_dash_bg_1280.png"),
            texture_atlas: Some(TextureAtlas {
                layout: bg_layout_handle,
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(320.0, 160.0, -100.0).with_scale(Vec3::splat(0.5)),
        Background,
        OnGameScreen
    ));
}

fn setup_diamonds(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // this should be per Level
    let diamond_count = 10;
    let diamond_length = 10.0;
    let mut random_gen = rand::rng();

    for _x in 0..diamond_count {
        let x_pos = random_gen.random_range(10..630);
        let y_pos = random_gen.random_range(10..310);

        commands.spawn((
            Mesh2d(meshes.add(bevy::math::primitives::Cuboid::from_length(diamond_length))),
            MeshMaterial2d(
                materials.add(ColorMaterial::from_color(Color::srgb(190.0, 190.0, 190.0))),
            ),
            Transform::from_xyz(x_pos as f32, y_pos as f32, 0.0),
            Diamond,
        ));
    }
}

fn collision_detection(
    enemy_query: Query<&Transform, With<Enemy>>,
    hero_query: Query<&Transform, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    //println!("Detect collision");
    let hero_transform = hero_query.single().unwrap();

    for enemy_transform in enemy_query.iter() {
        let distance = hero_transform
            .translation
            .distance(enemy_transform.translation);

        if distance < 30.0 {
            println!("Got hit: Game Over!");
            next_state.set(GameState::GameOver);
        }
    } // Placeholder for future collision detection logic
}

fn collision_detection_diamonds(
    mut commands: Commands,
    diamond_query: Query<(Entity, &Transform), With<Diamond>>,
    hero_query: Query<(&Transform,), With<Player>>,
    mut score: ResMut<Score>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    //println!("Detect collision");
    let hero_transform = hero_query.single().unwrap();

    for (diamond_entity, diamond_transform) in diamond_query.iter() {
        let distance = hero_transform
            .0
            .translation
            .distance(diamond_transform.translation);

        if distance < 30.0 {
            commands.entity(diamond_entity).despawn();
            // commands.insert_resource(Score {
            //     value: score.value + 1,
            // });
            score.increase();
        }
    }

    let diamonds_remaining = diamond_query.iter().count();
    //println!("Diamonds left; {}", diamonds_remaining);

    // if player collects all diamonds, move to next Level
}
