use bevy::prelude::*;

#[derive(Resource)]
pub struct Score {
    value: usize,
}

impl Score {
    pub fn increase(&mut self) {
        self.value += 1;
    }
}

impl Default for Score {
    fn default() -> Self {
        Score { value: 0 }
    }
}

#[derive(Component)]
pub(crate) struct ScoreDisplay;

pub(crate) fn setup_score_ui(mut commands: Commands) {
    println!("Setup score UI");
    // Top-level node for the UI
    commands
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Px(50.0),
            justify_content: JustifyContent::Center, // Left align horizontally
            align_items: AlignItems::Center,         // Top align vertically
            ..default()
        },))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center, // Left align horizontally
                        align_items: AlignItems::Center,         // Top align vertically
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    BorderRadius::all(Val::Px(8.)),
                    BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.5).into()),
                ))
                .with_children(|parent| {
                    // Text entity for the score
                    parent.spawn((
                        Text::new("score 0"),
                        TextFont {
                            font: Default::default(),
                            font_size: 20.0,
                            ..Default::default()
                        },
                        TextColor(Color::BLACK.into()),
                        ScoreDisplay, // Attach the marker component
                    ));
                });
        });
}

pub(crate) fn update_score_ui(
    mut commands: Commands, // Need Commands to re-insert the component
    score: Res<Score>,
    mut query: Query<(Entity, &Text), With<ScoreDisplay>>,
) {
    if score.is_changed() {
        if let Ok((entity, text_component)) = query.single_mut() {
            let updated_text_component = Text::new(format!("score {}", score.value));

            commands.entity(entity).insert(updated_text_component);
        }
    }
}
