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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Setup enemies");
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
