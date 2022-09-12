use crate::common::*;
use crate::game_textures::*;
use bevy::prelude::*;

fn delete_piece_physically(
    commands: &mut Commands,
    query: &mut Query<(Entity, &mut Position, &mut Transform, &mut Handle<Image>)>,
    position: Position,
) {
    for (entity, piece_position, _transform, _texture) in query.iter_mut() {
        if *piece_position != position {
            continue;
        }
        commands.entity(entity).despawn();
    }
}

pub fn move_piece_physically(
    game_textures: &mut Res<GameTextures>,
    commands: &mut Commands,
    query: &mut Query<(Entity, &mut Position, &mut Transform, &mut Handle<Image>)>,
    from: Position,
    to: Position,
    promote: bool,
) {
    delete_piece_physically(commands, query, to);
    for (_entity, mut piece_position, mut transform, mut texture) in query.iter_mut() {
        if *piece_position != from {
            continue;
        }
        if promote {
            if to.1 == 7 {
                *texture = game_textures.queenl.clone();
            } else {
                *texture = game_textures.queend.clone();
            }
        }
        *piece_position = to;
        transform.translation = real_piece_position(to);
    }
}
