use crate::game_textures::*;
use bevy::prelude::*;

pub const INFINITY: f32 = 1000000.;
pub const BIG_INFINITY: f32 = 10. * INFINITY;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct Position(pub i8, pub i8);

pub type Move = (Position, Position);

#[derive(Component)]
pub struct MainCamera;

pub struct MousePosition {
    pub position: Option<Position>,
}

pub struct SelectedSquare {
    pub position: Option<Position>,
}

pub fn real_position(position: Position) -> Vec3 {
    Vec3::new(
        (position.0 as f32 - 3.5) * IMAGE_SIZE.0,
        (position.1 as f32 - 3.5) * IMAGE_SIZE.1,
        0.,
    )
}

pub fn real_piece_position(position: Position) -> Vec3 {
    real_position(position) + Vec3::new(0., 0., 1.)
}

pub fn game_position(vec: Vec3) -> Option<Position> {
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
