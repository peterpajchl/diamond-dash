use crate::GameState;
use crate::despawn_screen;
use bevy::prelude::*;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), setup_menu)
            .add_systems(Update, button_system.run_if(in_state(GameState::GameOver)))
            .add_systems(OnExit(GameState::GameOver), despawn_screen::<OnGameOverScreen>);
    }
}

#[derive(Component)]
struct OnGameOverScreen;

// A marker component for menu buttons
#[derive(Component)]
enum MenuButtonAction {
    Ok,
}

fn setup_menu(mut commands: Commands) {
    println!("Setup menu");
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
        OnGameOverScreen,
    ));

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::WHITE),
            OnGameOverScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BorderRadius::all(Val::Px(8.0)),
                    BorderColor::all(Color::BLACK),
                    BackgroundColor(Color::WHITE),
                    TextColor(Color::BLACK),
                    MenuButtonAction::Ok,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("OK"),
                        TextFont {
                            font_size: 24.0,
                            font: Default::default(),
                            ..default()
                        },
                        TextColor::from(Color::BLACK),
                    ));
                });

            
        });
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut TextColor,
            &MenuButtonAction,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut exit: MessageWriter<AppExit>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    //println!("Handle buttons");
    for (interaction, mut color, mut border_color, mut text_color, menu_button_action) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::BLACK.into();
                border_color.set_all(Color::BLACK);
                *text_color = Color::WHITE.into();

                // Match on the button's action to trigger the correct event
                match menu_button_action {
                    MenuButtonAction::Ok => {
                        println!("Starting a new game!");
                        next_state.set(GameState::InGame);
                    }
                }
            }
            Interaction::Hovered => {
                *color = Color::BLACK.into();
                border_color.set_all(Color::BLACK);
                *text_color = Color::WHITE.into();
            }
            Interaction::None => {
                *color = Color::WHITE.into();
                border_color.set_all(Color::BLACK);
                *text_color = Color::BLACK.into();
            }
        }
    }
}
