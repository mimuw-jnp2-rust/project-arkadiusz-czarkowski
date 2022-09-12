use crate::common::*;
use crate::game_textures::*;
use bevy::prelude::*;

pub fn spawn_tile(
    commands: &mut Commands,
    texture: Handle<Image>,
    position: Position,
    highlight: bool,
) {
    let mut transform = Transform {
        translation: real_position(position),
        ..Default::default()
    };
    transform.scale *= SCALING_FACTOR;
    let mut sprite = SpriteBundle {
        texture,
        transform,
        ..Default::default()
    };
    if highlight {
        sprite.transform.translation += Vec3::new(0., 0., 0.5);
        sprite.sprite.color.set_a(0.7);
        commands.spawn_bundle(sprite).insert(Highlight {});
    } else {
        commands.spawn_bundle(sprite);
    }
}
