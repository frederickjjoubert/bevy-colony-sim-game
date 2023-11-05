use crate::colors::{BORDER_COLOR_NORMAL, NORMAL_BUTTON_COLOR, WHITE};
use crate::game::ui::components::*;
use crate::game::ui::fonts::FONT;
use bevy::prelude::*;

pub fn spawn_game_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::ColumnReverse,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            UI {},
            GameUI {},
            Name::new("Game UI"),
        ))
        .with_children(|game_ui| {
            game_ui
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(64.0),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::FlexStart,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        // background_color: BackgroundColor(Color::RED),
                        ..default()
                    },
                    UI {},
                    GameUI {},
                    Name::new("Bottom Bar"),
                ))
                .with_children(|bottom_bar| {
                    bottom_bar
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(100.0),
                                    height: Val::Percent(100.0),
                                    border: UiRect::all(Val::Px(4.0)),
                                    // margin: UiRect::all(Val::Px(16.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                border_color: BORDER_COLOR_NORMAL,
                                ..default()
                            },
                            UI {},
                            WallButton {},
                            Name::new("Wall Button"),
                        ))
                        .with_children(|quit_button| {
                            quit_button.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Wall",
                                        TextStyle {
                                            font: asset_server.load(FONT),
                                            font_size: 32.0,
                                            color: WHITE,
                                        },
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                });
        });
}

pub fn despawn_game_ui(mut commands: Commands, ui_query: Query<Entity, With<GameUI>>) {
    for ui_entity in ui_query.iter() {
        commands.entity(ui_entity).despawn_recursive();
    }
}
