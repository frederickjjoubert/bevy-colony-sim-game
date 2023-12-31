use crate::game::constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::game::utils::grid_to_world;
use bevy::prelude::*;
use bevy_hanabi::prelude::*;

pub const NUMBER_OF_STARS: u32 = 1000;
pub const RADIUS: f32 = 1000.0;

pub fn spawn_star_field(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    // Define a color gradient from white to transparent for the particles.
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1., 1., 1., 1.));
    gradient.add_key(1.0, Vec4::splat(0.));

    let writer = ExprWriter::new();

    // Create a lifetime modifier
    let lifetime = writer.lit(1_000_000_f32).expr(); // Very long lifetime
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let (world_x, world_y) = grid_to_world((MAP_WIDTH / 2) as i32, (MAP_HEIGHT / 2) as i32);

    // Create an initial position
    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::new(world_x, world_y, 0.0)).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        radius: writer.lit(RADIUS).expr(),
        dimension: ShapeDimension::Volume,
    };

    // Create an initial velocity
    let init_vel = SetVelocityCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        speed: writer.lit(0.1).expr(),
    };

    // let size = SetSizeModifier {
    //     size: CpuValue::Single(Vec2::splat(0.1)),
    //     screen_space_size: false,
    // };

    // Create a spawner
    let spawner = Spawner::once((NUMBER_OF_STARS as f32).into(), true);

    // Create the effect asset
    // let effect = effects.add();
    let effect = EffectAsset::new(NUMBER_OF_STARS, spawner, writer.finish())
        .with_name("star_field")
        .init(init_pos)
        .init(init_vel)
        .init(init_lifetime)
        // .render(size)
        .render(ColorOverLifetimeModifier { gradient });

    let effect_handle = effects.add(effect);

    // Spawn an entity with a ParticleEffect component referencing the asset
    commands.spawn(ParticleEffectBundle {
        effect: ParticleEffect::new(effect_handle).with_z_layer_2d(Some(0.1)),
        ..Default::default()
    });
}

// References
// 1. Bevy Hanabi 2d Example
// https://github.com/djeedai/bevy_hanabi/blob/main/examples/2d.rs
