use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub(crate) struct Enemy;

#[derive(Component)]
pub(crate) struct EnemyMovement {
    direction: Vec2,
    speed: f32,
}

pub(crate) fn setup_enemies(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    println!("Setup enemies");
    let initial_enemies_count = 10;
    let enemy_speed = 50.0;
    let mut random_gen = rand::rng();

    let texture = asset_server.load::<Image>("sprites/characters/enemy/idle/diamond_dash_monster_white_s.png");
    let atlas = TextureAtlasLayout::from_grid(UVec2::new(48, 64), 8, 6, None, None);

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
                image: texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layouts.add(atlas.clone()),
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
        ));
    }
}

pub(crate) fn enemies_movement(
    mut query: Query<(&mut Transform, &mut EnemyMovement)>,
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
