use crate::common::*;
use crate::game_state::*;
use crate::game_textures::*;
use crate::piece::*;
use crate::program_options::*;
use bevy::prelude::*;
use debug_print::debug_println;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let game_textures = GameTextures {
        tilel: asset_server.load(TILEL_SPRITE),
        kingl: asset_server.load(KINGL_SPRITE),
        queenl: asset_server.load(QUEENL_SPRITE),
        rookl: asset_server.load(ROOKL_SPRITE),
        bishopl: asset_server.load(BISHOPL_SPRITE),
        knightl: asset_server.load(KNIGHTL_SPRITE),
        pawnl: asset_server.load(PAWNL_SPRITE),
        tiled: asset_server.load(TILED_SPRITE),
        kingd: asset_server.load(KINGD_SPRITE),
        queend: asset_server.load(QUEEND_SPRITE),
        rookd: asset_server.load(ROOKD_SPRITE),
        bishopd: asset_server.load(BISHOPD_SPRITE),
        knightd: asset_server.load(KNIGHTD_SPRITE),
        pawnd: asset_server.load(PAWND_SPRITE),
        highlight: asset_server.load(HIGHLIGHT_SPRITE),
    };
    commands.insert_resource(game_textures);

    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(MainCamera);

    commands.insert_resource(MousePosition { position: None });

    commands.insert_resource(SelectedSquare { position: None });

    let mut player_moves = rand::random::<bool>();
    unsafe {
        if NUMBER_OF_PLAYERS == 0 {
            player_moves = false;
        }
        if NUMBER_OF_PLAYERS == 2 {
            player_moves = true;
        }
    }
    if player_moves {
        debug_println!("Your move");
    }
    commands.insert_resource(GameState {
        board: [[None; 8]; 8],
        now_moves: PieceColor::White,
        player_moves,
    });
}
