use bevy::prelude::*;
use crate::game::AnimationIndices;
use crate::game::AnimationData;

#[derive(Component)]
pub(crate) struct Player;

#[derive(Component, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum PlayerDirection {
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
pub(crate) enum PlayerAnimationState {
    Idle,
    Walk,
    Jump,
    Run,
}

#[derive(Resource)]
pub(crate) struct PlayerAnimationData {
    idle: AnimationData,
    walk: AnimationData,
    jump: AnimationData,
    run: AnimationData,
}

impl PlayerAnimationData {
    pub fn new(
        idle: AnimationData,
        walk: AnimationData,
        jump: AnimationData,
        run: AnimationData,
    ) -> Self {
        Self {
            idle,
            walk,
            jump,
            run,
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub(crate) struct AnimationTimer(Timer);

pub(crate) fn setup_hero(mut commands: Commands, animation_data: Res<PlayerAnimationData>) {
    println!("Setup hero");

    commands.spawn((
        Sprite {
            image: animation_data.idle.texture.clone(), // Start with the idle texture
            texture_atlas: Some(TextureAtlas {
                layout: animation_data.idle.texture_atlas.clone(),
                index: animation_data.idle.frames.first,
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

pub(crate) fn player_movement(
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
    //println!("Move player");
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

pub(crate) fn update_player_animation(
    mut query: Query<(
        &PlayerAnimationState,
        &PlayerDirection,
        &mut Sprite,
        &mut AnimationTimer,
    )>,
    player_animation_data: Res<PlayerAnimationData>, // Access the animation data
) {
    //println!("Update player animation");
    for (current_state, player_direction, mut sprite, mut animation_timer) in &mut query {
        // Change animation data if the state has changed
        let new_texture_handle = match *current_state {
            PlayerAnimationState::Idle => player_animation_data.idle.texture.clone(),
            PlayerAnimationState::Walk => player_animation_data.walk.texture.clone(),
            PlayerAnimationState::Jump => player_animation_data.jump.texture.clone(),
            PlayerAnimationState::Run => player_animation_data.run.texture.clone(),
        };

        let new_atlas_layout_handle = match *current_state {
            PlayerAnimationState::Idle => player_animation_data.idle.texture_atlas.clone(),
            PlayerAnimationState::Walk => player_animation_data.walk.texture_atlas.clone(),
            PlayerAnimationState::Jump => player_animation_data.jump.texture_atlas.clone(),
            PlayerAnimationState::Run => player_animation_data.run.texture_atlas.clone(),
        };

        let new_animation_indices = match *current_state {
            PlayerAnimationState::Idle => player_animation_data.idle.frames.clone(),
            PlayerAnimationState::Walk => {
                match player_direction {
                    PlayerDirection::Down => player_animation_data.walk.frames.clone(),
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
                    _ => player_animation_data.walk.frames.clone(), // Default to walk frames
                }
            }
            PlayerAnimationState::Jump => player_animation_data.jump.frames.clone(),
            PlayerAnimationState::Run => player_animation_data.run.frames.clone(),
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
        &PlayerAnimationState,
        &PlayerDirection,
    )>,
    player_animation_data: Res<PlayerAnimationData>,
) {
    //println!("Animate sprites");
    for (mut timer, mut sprite, animation_state, player_direction) in &mut query {
        timer.tick(time.delta());

        // Determine correct indices for current animation and direction
        let indices = match *animation_state {
            PlayerAnimationState::Idle => player_animation_data.idle.frames.clone(),
            PlayerAnimationState::Walk => match player_direction {
                PlayerDirection::Down => player_animation_data.walk.frames.clone(),
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
                _ => player_animation_data.walk.frames.clone(),
            },
            PlayerAnimationState::Jump => player_animation_data.jump.frames.clone(),
            PlayerAnimationState::Run => player_animation_data.run.frames.clone(),
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
