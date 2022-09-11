use crate::common::*;
use crate::game_textures::*;
use bevy::prelude::*;

pub fn real_pos(position: Position) -> Vec3 {
    Vec3::new(
        (position.0 as f32 - 3.5) * IMAGE_SIZE.0,
        (position.1 as f32 - 3.5) * IMAGE_SIZE.1,
        0.,
    )
}

pub fn real_piece_pos(position: Position) -> Vec3 {
    real_pos(position) + Vec3::new(0., 0., 1.)
}

pub fn game_pos(vec: Vec3) -> Option<Position> {
    let (mut x, mut y) = (vec.x, vec.y);
    x += 3.5 * IMAGE_SIZE.0;
    y += 3.5 * IMAGE_SIZE.1;
    x /= IMAGE_SIZE.0;
    y /= IMAGE_SIZE.0;
    x = x.round();
    y = y.round();
    if x < 0.0 || y < 0.0 || x >= 8.0 || y >= 8.0 {
        None
    } else {
        Some(Position(x as i8, y as i8))
    }
}

#[derive(Component)]
pub struct Highlight {}

pub fn delete_highlight(commands: &mut Commands, query: &Query<Entity, With<Highlight>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_tile(
    commands: &mut Commands,
    texture: Handle<Image>,
    position: Position,
    highlight: bool,
) {
    let mut transform = Transform {
        translation: real_pos(position),
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
