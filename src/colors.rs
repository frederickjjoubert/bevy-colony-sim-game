#![allow(unused)]

use bevy::prelude::Color;
use bevy::ui::BorderColor;

#[warn(unused_variables)]
pub const NORMAL_BUTTON_COLOR: Color = BLACK_2;
pub const HOVERED_BUTTON_COLOR: Color = YELLOW;
pub const PRESSED_BUTTON_COLOR: Color = GREEN_1;

pub const BORDER_COLOR_NORMAL: BorderColor = BorderColor(WHITE);

// Green
pub const GREEN_1: Color = Color::rgb(211.0 / 255.0, 252.0 / 255.0, 126.0 / 255.0);

// White
pub const WHITE: Color = Color::rgb(249.0 / 255.0, 230.0 / 255.0, 207.0 / 255.0);

// Black
pub const BLACK_1: Color = Color::rgb(19.0 / 255.0, 19.0 / 255.0, 19.0 / 255.0);
pub const BLACK_2: Color = Color::rgb(27.0 / 255.0, 27.0 / 255.0, 27.0 / 255.0);

// Yellow
pub const YELLOW: Color = Color::rgb(255.0 / 255.0, 235.0 / 255.0, 87.0 / 255.0);
