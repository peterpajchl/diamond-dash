use bevy::prelude::*;
use rand::Rng;
use crate::game::AnimationIndices;
use crate::game::AnimationData;

#[derive(Component)]
pub(crate) struct Enemy;

#[derive(Component)]
pub(crate) struct EnemyMovement {
    direction: Vec2,
    speed: f32,
}

#[derive(Component, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum EnemyDirection {
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

#[derive(Component, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum EnemyAnimationState {
    Walk,
}

#[derive(Resource)]
pub(crate) struct EnemyAnimationData {
    walk: AnimationData,
}

impl EnemyAnimationData {
    pub fn new(
        walk: AnimationData,
    ) -> Self {
        Self {
            walk,
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub(crate) struct AnimationTimer(Timer);

pub(crate) fn setup_enemies(
    mut commands: Commands,
    animation_data: Res<EnemyAnimationData>
) {
    println!("Setup enemies");
    let initial_enemies_count = 1;//10;
    let enemy_speed = 50.0;
    let mut random_gen = rand::rng();

    // Hero spawn position (middle of screen)
    let hero_spawn_x = 320.0;
    let hero_spawn_y = 160.0;
    let safe_zone_radius = 100.0; // Adjust this to control exclusion area size


    for _ in 0..initial_enemies_count {

        let mut x_pos;
        let mut y_pos;
        // Keep generating random positions until one is outside the safe zone
        // For now, we should find something more determnistic later
        loop {
            x_pos = random_gen.random_range(10..630) as f32;
            y_pos = random_gen.random_range(10..310) as f32;
            
            // Calculate distance from hero spawn point
            let distance = ((x_pos - hero_spawn_x).powi(2) + (y_pos - hero_spawn_y).powi(2)).sqrt();
            
            // Accept position if it's outside the safe zone
            if distance > safe_zone_radius {
                break;
            }
        }

        let direction = if random_gen.random_bool(0.5) {
            1.0
        } else {
            -1.0
        };

        commands.spawn((
            Sprite {
                image: animation_data.walk.texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: animation_data.walk.texture_atlas.clone(),
                    index: 0,
                }),
                ..default()
            },
            Transform::from_xyz(x_pos as f32, y_pos as f32, 0.0).with_scale(Vec3::splat(1.0)),
            Enemy,
            EnemyMovement {
                direction: Vec2::new(direction, direction),
                speed: enemy_speed,
            },
            EnemyAnimationState::Walk,
            EnemyDirection::Down,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
        ));
    }
}

pub(crate) fn enemies_movement(
    mut query: Query<(&mut Transform, &mut EnemyMovement, &mut EnemyAnimationState, &mut EnemyDirection)>,
    window_query: Query<&Window, With<Window>>,
    time: Res<Time>,
) {
    //println!("Move enemies");
    let window = window_query.single().unwrap();
    let window_width = window.width();
    let window_height = window.height();

    // Calculate the boundaries of the playable area
    let x_min = 0.0;
    let x_max = window_width;
    let y_min = 0.0;
    let y_max = window_height;

    for (mut transform, mut enemy_movement, mut animation_state, mut enemy_direction) in &mut query {
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
        *enemy_direction = if enemy_movement.direction.x > 0.0 && enemy_movement.direction.y > 0.0 {
            EnemyDirection::RightUp
        } else if enemy_movement.direction.x > 0.0 && enemy_movement.direction.y < 0.0 {
            EnemyDirection::RightDown
        } else if enemy_movement.direction.x < 0.0 && enemy_movement.direction.y > 0.0 {
            EnemyDirection::LeftUp
        } else if enemy_movement.direction.x < 0.0 && enemy_movement.direction.y < 0.0 {
            EnemyDirection::LeftDown
        } else if enemy_movement.direction.x > 0.0 {
            EnemyDirection::Right
        } else if enemy_movement.direction.x < 0.0 {
            EnemyDirection::Left
        } else if enemy_movement.direction.y > 0.0 {
            EnemyDirection::Up
        } else if enemy_movement.direction.y < 0.0 {
            EnemyDirection::Down
        } else {
            EnemyDirection::None
        };
    }
}

pub(crate) fn update_enemy_animation(
    mut query: Query<(
        &EnemyAnimationState,
        &EnemyDirection,
        &mut Sprite,
        &mut AnimationTimer,
    )>,
    player_animation_data: Res<EnemyAnimationData>, // Access the animation data
) {
    //println!("Update player animation");
    for (current_state, player_direction, mut sprite, mut animation_timer) in &mut query {
        // Change animation data if the state has changed
        let new_texture_handle = match *current_state {
            EnemyAnimationState::Walk => player_animation_data.walk.texture.clone(),
        };

        let new_atlas_layout_handle = match *current_state {
            EnemyAnimationState::Walk => player_animation_data.walk.texture_atlas.clone(),
        };

        let new_animation_indices = match *current_state {
            EnemyAnimationState::Walk => {
                match player_direction {
                    EnemyDirection::Down => player_animation_data.walk.frames.clone(),
                    EnemyDirection::Up => AnimationIndices {
                        first: 24,
                        last: 31,
                    },
                    EnemyDirection::Left => AnimationIndices { first: 8, last: 15 },
                    EnemyDirection::Right => AnimationIndices {
                        first: 40,
                        last: 47,
                    },
                    EnemyDirection::LeftUp => AnimationIndices {
                        first: 16,
                        last: 23,
                    },
                    EnemyDirection::RightUp => AnimationIndices {
                        first: 32,
                        last: 39,
                    },
                    EnemyDirection::LeftDown => AnimationIndices { first: 8, last: 15 },
                    EnemyDirection::RightDown => AnimationIndices {
                        first: 40,
                        last: 47,
                    },
                    _ => player_animation_data.walk.frames.clone(), // Default to walk frames
                }
            }
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

pub(crate) fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut Sprite,
        &EnemyAnimationState,
        &EnemyDirection,
    )>,
    enemy_animation_data: Res<EnemyAnimationData>,
) {
    //println!("Animate sprites");
    for (mut timer, mut sprite, animation_state, player_direction) in &mut query {
        timer.tick(time.delta());

        // Determine correct indices for current animation and direction
        let indices = match *animation_state {
            EnemyAnimationState::Walk => match player_direction {
                EnemyDirection::Down => enemy_animation_data.walk.frames.clone(),
                EnemyDirection::Up => AnimationIndices {
                    first: 24,
                    last: 31,
                },
                EnemyDirection::Left => AnimationIndices { first: 8, last: 15 },
                EnemyDirection::Right => AnimationIndices {
                    first: 40,
                    last: 47,
                },
                EnemyDirection::LeftUp => AnimationIndices {
                    first: 16,
                    last: 23,
                },
                EnemyDirection::LeftDown => AnimationIndices { first: 8, last: 15 },
                EnemyDirection::RightUp => AnimationIndices {
                    first: 32,
                    last: 39,
                },
                EnemyDirection::RightDown => AnimationIndices {
                    first: 40,
                    last: 47,
                },
                _ => enemy_animation_data.walk.frames.clone(),
            },
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
