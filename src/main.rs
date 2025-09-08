use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::WindowResolution;
use rand::Rng;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct EnemyMovement {
    direction: Vec2,
    speed: f32,
}

#[derive(Component, Clone)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum PlayerAnimationState {
    Idle,
    Walk,
    Jump,
    Run,
}

#[derive(Component, Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum PlayerDirection {
    Left,
    LeftUp,
    LeftDown,
    Right,
    RightUp,
    RightDown,
    Up,
    Down,
    None,
}

// Store animation details in a resource or a component on the entity
#[derive(Resource)]
struct PlayerAnimationData {
    //  Handles to TextureAtlasLayouts for each animation
    idle_atlas: Handle<TextureAtlasLayout>,
    walk_atlas: Handle<TextureAtlasLayout>,
    jump_atlas: Handle<TextureAtlasLayout>,
    run_atlas: Handle<TextureAtlasLayout>,

    //  Frame indices for each animation segment
    idle_frames: AnimationIndices,
    walk_frames: AnimationIndices,
    jump_frames: AnimationIndices,
    run_frames: AnimationIndices,

    //  Handles to the textures for each animation
    idle_texture: Handle<Image>,
    walk_texture: Handle<Image>,
    jump_texture: Handle<Image>,
    run_texture: Handle<Image>,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Diamond Dash".into(),
                        resolution: WindowResolution::new(640.0, 320.0),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_enemies)
        .add_systems(
            Update,
            (player_movement, animate_sprite, update_player_animation),
        )
        .add_systems(Update, enemies_movement)
        .run();
}

fn setup_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let initial_enemies_count = 10;
    let radius = 10.0;
    let enemy_speed = 50.0;
    let mut random_gen = rand::rng();

    for x in 0..initial_enemies_count {
        let x_pos = random_gen.random_range(10..630);
        let y_pos = random_gen.random_range(10..310);

        let colour = Color::srgb(
            (x as f32) / initial_enemies_count as f32,
            1.0 - (x as f32) / initial_enemies_count as f32,
            0.5,
        );

        let direction = if random_gen.random_bool(0.5) {
            1.0
        } else {
            -1.0
        };

        commands.spawn((
            Mesh2d(meshes.add(Circle::new(radius))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(colour))),
            Transform::from_xyz(x_pos as f32, y_pos as f32, 0.0),
            Enemy,
            EnemyMovement {
                direction: Vec2::new(direction, direction),
                speed: enemy_speed,
            },
        ));
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load textures for all animations
    let texture_idle = asset_server.load("sprites/characters/hero/idle/idle.png");
    let texture_walk = asset_server.load("sprites/characters/hero/walk/walk.png");
    let texture_run = asset_server.load("sprites/characters/hero/dash/dash.png");
    let texture_jump = asset_server.load("sprites/characters/hero/jump/normal/jump.png");

    // Create texture atlases for each animation
    let idle_atlas_layout = TextureAtlasLayout::from_grid(UVec2::new(48, 64), 8, 6, None, None);
    let walk_atlas_layout = TextureAtlasLayout::from_grid(UVec2::new(48, 64), 8, 6, None, None);
    let run_atlas_layout = TextureAtlasLayout::from_grid(UVec2::new(48, 64), 8, 6, None, None);
    let jump_atlas_layout = TextureAtlasLayout::from_grid(UVec2::new(48, 64), 8, 6, None, None);

    let idle_atlas_handle = texture_atlas_layouts.add(idle_atlas_layout);
    let walk_atlas_handle = texture_atlas_layouts.add(walk_atlas_layout);
    let run_atlas_handle = texture_atlas_layouts.add(run_atlas_layout);
    let jump_atlas_handle = texture_atlas_layouts.add(jump_atlas_layout);

    let idle_frames = AnimationIndices { first: 0, last: 7 };
    let walk_frames = AnimationIndices { first: 0, last: 7 };
    let jump_frames = AnimationIndices { first: 0, last: 7 };
    let run_frames = AnimationIndices { first: 0, last: 7 };

    commands.insert_resource(PlayerAnimationData {
        idle_atlas: idle_atlas_handle.clone(),
        walk_atlas: walk_atlas_handle,
        jump_atlas: jump_atlas_handle,
        run_atlas: run_atlas_handle,
        idle_frames: idle_frames.clone(),
        walk_frames: walk_frames.clone(),
        jump_frames: jump_frames.clone(),
        run_frames: run_frames.clone(),
        idle_texture: texture_idle.clone(),
        walk_texture: texture_walk.clone(),
        jump_texture: texture_jump.clone(),
        run_texture: texture_run.clone(),
    });

    // Camera
    let camera = Camera2d::default();

    commands.spawn((
        camera,
        Transform {
            translation: Vec3::new(640.0 / 2.0, 320.0 / 2.0, 0.0),
            ..default()
        },
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMax {
                max_width: 640.0,
                max_height: 320.0,
            },
            scale: 1.0,
            ..OrthographicProjection::default_2d()
        }),
    ));

    // Spawn Hero
    commands.spawn((
        Sprite {
            image: texture_idle, // Start with the idle texture
            texture_atlas: Some(TextureAtlas {
                layout: idle_atlas_handle,
                index: idle_frames.first,
            }),
            ..default()
        },
        Transform::from_xyz(640.0 / 2.0, 320.0 / 2.0, 0.0).with_scale(Vec3::splat(1.0)),
        Player,
        PlayerAnimationState::Idle,
        PlayerDirection::None,
        AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating)),
    ));
}

fn enemies_movement(
    mut query: Query<(&mut Transform, &mut EnemyMovement)>,
    window_query: Query<&Window, With<Window>>,
    time: Res<Time>,
) {
    let window = window_query.single().unwrap();
    let window_width = window.width();
    let window_height = window.height();

    // Calculate the boundaries of the playable area
    let x_min = 0.0;
    let x_max = window_width;
    let y_min = 0.0;
    let y_max = window_height;
    dbg!(x_min, x_max, y_min, y_max);
    for (mut transform, mut enemy_movement) in &mut query {
        let mut translation = transform.translation;

        // Update position based on current direction and speed
        translation.x += enemy_movement.direction.x * enemy_movement.speed * time.delta_secs();
        translation.y += enemy_movement.direction.y * enemy_movement.speed * time.delta_secs();

        // Check for collision with horizontal window edges
        if translation.x > x_max || translation.x < x_min {
            enemy_movement.direction.x *= -1.0; // Reverse horizontal direction
        }

        // Check for collision with vertical window edges
        if translation.y > y_max || translation.y < y_min {
            enemy_movement.direction.y *= -1.0; // Reverse vertical direction
        }

        // Apply the new translation
        transform.translation = translation;
    }
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (
            &mut Transform,
            &mut PlayerAnimationState,
            &mut PlayerDirection,
        ),
        With<Player>,
    >,
    time: Res<Time>,
) {
    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    if direction != Vec2::ZERO {
        direction = direction; //normalize(); // Normalize the direction vector
    }

    let base_speed = 100.0;

    let speed = if keyboard_input.pressed(KeyCode::ShiftLeft) {
        base_speed * 1.5 // Running speed
    } else {
        base_speed
    };

    for (mut transform, mut animation_state, mut player_direction) in &mut query {
        transform.translation += Vec3::new(
            direction.x * speed * time.delta_secs(),
            direction.y * speed * time.delta_secs(),
            0.0,
        );

        // Update animation state based on movement
        if direction != Vec2::ZERO {
            if keyboard_input.pressed(KeyCode::ShiftLeft) {
                if *animation_state != PlayerAnimationState::Run {
                    *animation_state = PlayerAnimationState::Run;
                }
            } else if keyboard_input.pressed(KeyCode::Space) {
                if *animation_state != PlayerAnimationState::Jump {
                    *animation_state = PlayerAnimationState::Jump;
                }
            } else {
                if *animation_state != PlayerAnimationState::Walk {
                    *animation_state = PlayerAnimationState::Walk;
                }
            }

            // direction
            if keyboard_input.pressed(KeyCode::ArrowLeft)
                && keyboard_input.pressed(KeyCode::ArrowUp)
            {
                *player_direction = PlayerDirection::LeftUp;
            } else if keyboard_input.pressed(KeyCode::ArrowLeft)
                && keyboard_input.pressed(KeyCode::ArrowDown)
            {
                *player_direction = PlayerDirection::LeftDown;
            } else if keyboard_input.pressed(KeyCode::ArrowRight)
                && keyboard_input.pressed(KeyCode::ArrowUp)
            {
                *player_direction = PlayerDirection::RightUp;
            } else if keyboard_input.pressed(KeyCode::ArrowRight)
                && keyboard_input.pressed(KeyCode::ArrowDown)
            {
                *player_direction = PlayerDirection::RightDown;
            } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
                *player_direction = PlayerDirection::Left;
            } else if keyboard_input.pressed(KeyCode::ArrowRight) {
                *player_direction = PlayerDirection::Right;
            } else if keyboard_input.pressed(KeyCode::ArrowUp) {
                *player_direction = PlayerDirection::Up;
            } else if keyboard_input.pressed(KeyCode::ArrowDown) {
                *player_direction = PlayerDirection::Down;
            } else {
                *player_direction = PlayerDirection::None; // Reset direction if no arrow keys are pressed
            }
        } else {
            if *animation_state != PlayerAnimationState::Idle {
                *animation_state = PlayerAnimationState::Idle;
            }

            *player_direction = PlayerDirection::None; // Reset direction when idle
        }
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut Sprite,
        &PlayerAnimationState,
        &PlayerDirection,
    )>,
    player_animation_data: Res<PlayerAnimationData>,
) {
    for (mut timer, mut sprite, animation_state, player_direction) in &mut query {
        timer.tick(time.delta());

        // Determine correct indices for current animation and direction
        let indices = match *animation_state {
            PlayerAnimationState::Idle => player_animation_data.idle_frames.clone(),
            PlayerAnimationState::Walk => match player_direction {
                PlayerDirection::Down => player_animation_data.walk_frames.clone(),
                PlayerDirection::Up => AnimationIndices {
                    first: 24,
                    last: 31,
                },
                PlayerDirection::Left => AnimationIndices { first: 8, last: 15 },
                PlayerDirection::Right => AnimationIndices {
                    first: 40,
                    last: 47,
                },
                PlayerDirection::LeftUp => AnimationIndices {
                    first: 16,
                    last: 23,
                },
                PlayerDirection::LeftDown => AnimationIndices { first: 8, last: 15 },
                PlayerDirection::RightUp => AnimationIndices {
                    first: 32,
                    last: 39,
                },
                PlayerDirection::RightDown => AnimationIndices {
                    first: 40,
                    last: 47,
                },
                _ => player_animation_data.walk_frames.clone(),
            },
            PlayerAnimationState::Jump => player_animation_data.jump_frames.clone(),
            PlayerAnimationState::Run => player_animation_data.run_frames.clone(),
        };

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index >= indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}

fn update_player_animation(
    mut query: Query<(
        &PlayerAnimationState,
        &PlayerDirection,
        &mut Sprite,
        &mut AnimationTimer,
    )>,
    player_animation_data: Res<PlayerAnimationData>, // Access the animation data
) {
    for (current_state, player_direction, mut sprite, mut animation_timer) in &mut query {
        // Change animation data if the state has changed
        let new_texture_handle = match *current_state {
            PlayerAnimationState::Idle => player_animation_data.idle_texture.clone(),
            PlayerAnimationState::Walk => player_animation_data.walk_texture.clone(),
            PlayerAnimationState::Jump => player_animation_data.jump_texture.clone(),
            PlayerAnimationState::Run => player_animation_data.run_texture.clone(),
        };

        let new_atlas_layout_handle = match *current_state {
            PlayerAnimationState::Idle => player_animation_data.idle_atlas.clone(),
            PlayerAnimationState::Walk => player_animation_data.walk_atlas.clone(),
            PlayerAnimationState::Jump => player_animation_data.jump_atlas.clone(),
            PlayerAnimationState::Run => player_animation_data.run_atlas.clone(),
        };

        let new_animation_indices = match *current_state {
            PlayerAnimationState::Idle => player_animation_data.idle_frames.clone(),
            PlayerAnimationState::Walk => {
                match player_direction {
                    PlayerDirection::Down => player_animation_data.walk_frames.clone(),
                    PlayerDirection::Up => AnimationIndices {
                        first: 24,
                        last: 31,
                    },
                    PlayerDirection::Left => AnimationIndices { first: 8, last: 15 },
                    PlayerDirection::Right => AnimationIndices {
                        first: 40,
                        last: 47,
                    },
                    PlayerDirection::LeftUp => AnimationIndices {
                        first: 16,
                        last: 23,
                    },
                    PlayerDirection::RightUp => AnimationIndices {
                        first: 32,
                        last: 39,
                    },
                    PlayerDirection::LeftDown => AnimationIndices { first: 8, last: 15 },
                    PlayerDirection::RightDown => AnimationIndices {
                        first: 40,
                        last: 47,
                    },
                    _ => player_animation_data.walk_frames.clone(), // Default to walk frames
                }
            }
            PlayerAnimationState::Jump => player_animation_data.jump_frames.clone(),
            PlayerAnimationState::Run => player_animation_data.run_frames.clone(),
        };

        if sprite.texture_atlas.is_none()
            || sprite.texture_atlas.as_ref().unwrap().layout != new_atlas_layout_handle
        {
            if let Some(atlas) = &mut sprite.texture_atlas {
                // Update the sprite's texture atlas and index
                atlas.layout = new_atlas_layout_handle;
                atlas.index = new_animation_indices.first; // Reset to the first frame of the new animation
            }

            sprite.image = new_texture_handle;
            animation_timer.reset(); // Reset animation timer
        }
    }
}
