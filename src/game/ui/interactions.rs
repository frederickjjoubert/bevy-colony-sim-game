use crate::colors::*;
use crate::game::ui::components::WallButton;
use bevy::prelude::*;

pub fn wall_button_interactions(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<WallButton>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut background_color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                text.sections[0].style.color = BLACK_1;
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
                text.sections[0].style.color = BLACK_1;
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
                text.sections[0].style.color = WHITE;
            }
        }
    }
}
