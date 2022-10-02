use crate::common::*;
use crate::game_state::*;
use crate::game_textures::*;
use crate::program_options::*;
use bevy::prelude::*;
use debug_print::debug_println;
use rand::seq::SliceRandom;
use std::collections::HashMap;

pub fn computer_moves_system(
    query_highlight: Query<Entity, With<Highlight>>,
    game_textures: Res<GameTextures>,
    mut commands: Commands,
    query: Query<(Entity, &mut Position, &mut Transform, &mut Handle<Image>)>,
    mut game_state: ResMut<GameState>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    let (white_king, black_king, _is_endgame) = game_state.stats();
    if !white_king {
        println!("Black wins!");
        std::thread::sleep(std::time::Duration::from_millis(1000));
        app_exit_events.send(bevy::app::AppExit);
    } else if !black_king {
        println!("White wins!");
        std::thread::sleep(std::time::Duration::from_millis(1000));
        app_exit_events.send(bevy::app::AppExit);
    } else if !game_state.player_moves {
        debug_println!("Thinking ...");
        let mut cache = HashMap::<(GameState, i32), f32>::new();
        let score: f32;
        unsafe {
            score = game_state.evaluate(DEPTH, &mut cache, -BIG_INFINITY, BIG_INFINITY);
        }
        let possible_moves = game_state.generate_legal_moves();
        let good_moves = possible_moves
            .into_iter()
            .filter(|(from, to)| {
                let mut next_state = game_state.clone();
                next_state.move_piece(*from, *to, true);
                let next_state_score: f32;
                unsafe {
                    next_state_score =
                        next_state.evaluate(DEPTH - 1, &mut cache, -BIG_INFINITY, BIG_INFINITY);
                }
                debug_println!("(move, score) = ({:?}, {:?})", (from, to), next_state_score);
                score == next_state_score
            })
            .collect::<Vec<Move>>();
        debug_println!("score = {}", score);
        debug_println!("good moves = {:?}", good_moves);
        debug_println!("cache size: {}", cache.len());
        delete_highlight(&mut commands, &query_highlight);
        let computer_move = good_moves.choose(&mut rand::thread_rng());
        if let Some(&(from, to)) = computer_move {
            game_state.computer_move(game_textures, &mut commands, query, from, to);
            if game_state.player_moves {
                debug_println!("Your move");
            }
        } else {
            app_exit_events.send(bevy::app::AppExit);
        }
    }
}
