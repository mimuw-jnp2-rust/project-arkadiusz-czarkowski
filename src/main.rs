// TODO position zdaje się w ogóle nie działać :(

use bevy::prelude::*;
use bevy_mod_picking::*;

const IMAGE_SIZE: (f32, f32) = (45., 45.);

const TILEL_SPRITE: &str = "sprites/tilel.png";
const KINGL_SPRITE: &str = "sprites/kl.png";
const QUEENL_SPRITE: &str = "sprites/ql.png";
const ROOKL_SPRITE: &str = "sprites/rl.png";
const BISHOPL_SPRITE: &str = "sprites/bl.png";
const KNIGHTL_SPRITE: &str = "sprites/nl.png";
const PAWNL_SPRITE: &str = "sprites/pl.png";
const TILED_SPRITE: &str = "sprites/tiled.png";
const KINGD_SPRITE: &str = "sprites/kd.png";
const QUEEND_SPRITE: &str = "sprites/qd.png";
const ROOKD_SPRITE: &str = "sprites/rd.png";
const BISHOPD_SPRITE: &str = "sprites/bd.png";
const KNIGHTD_SPRITE: &str = "sprites/nd.png";
const PAWND_SPRITE: &str = "sprites/pd.png";

struct GameTextures {
    tilel: Handle<Image>,
    kingl: Handle<Image>,
    queenl: Handle<Image>,
    rookl: Handle<Image>,
    bishopl: Handle<Image>,
    knightl: Handle<Image>,
    pawnl: Handle<Image>,
    tiled: Handle<Image>,
    kingd: Handle<Image>,
    queend: Handle<Image>,
    rookd: Handle<Image>,
    bishopd: Handle<Image>,
    knightd: Handle<Image>,
    pawnd: Handle<Image>,
}

enum PieceColor {
    White,
    Black,
}

enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Component)]
struct Piece {
    color: PieceColor,
    piece_type: PieceType,
    x: i8,
    y: i8,
}

impl Piece {
    fn can_move_king(&self, x: i8, y: i8) -> bool {
        i8::abs(self.x - x) <= 1 && i8::abs(self.y - y) <= 1
    }

    fn can_move_queen(&self, x: i8, y: i8) -> bool {
        self.can_move_rook(x, y) || self.can_move_bishop(x, y)
    }

    fn can_move_rook(&self, x: i8, y: i8) -> bool {
        true
    }

    fn can_move_bishop(&self, x: i8, y: i8) -> bool {
        true
    }

    fn can_move_knight(&self, x: i8, y: i8) -> bool {
        (i8::abs(self.x - x) == 1 && i8::abs(self.y - y) == 2)
            || (i8::abs(self.x - x) == 2 && i8::abs(self.y - y) == 1)
    }

    fn can_move_pawn(&self, x: i8, y: i8) -> bool {
        true
    }

    fn can_move(&self, x: i8, y: i8) -> bool { // ignores attacks on the king and pins
        // add a check for the square being occupied by a piece of the same color
        match self.piece_type {
            PieceType::King => self.can_move_king(x, y),
            PieceType::Queen => self.can_move_queen(x, y),
            PieceType::Rook => self.can_move_rook(x, y),
            PieceType::Bishop => self.can_move_bishop(x, y),
            PieceType::Knight => self.can_move_knight(x, y),
            PieceType::Pawn => self.can_move_pawn(x, y),
        }
    }

    fn move_piece(&mut self, x: i8, y: i8) {
        self.x = x;
        self.y = y;
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
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
    };
    commands.insert_resource(game_textures);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert_bundle(PickingCameraBundle::default());
}

fn create_board(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
) {
    let tilel = game_textures.tilel.clone();
    let tiled = game_textures.tiled.clone();
    for i in 0..8 {
        for j in 0..8 {
            let tile = if (i + j) % 2 == 0 { &tiled } else { &tilel };
            spawn_tile(
                &mut commands,
                tile.clone(),
                (i, j)
            );
        }
    }
}

fn create_pieces(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
) {
    // kingl
    spawn_piece(
        &mut commands,
        game_textures.kingl.clone(),
        (4, 0),
        PieceColor::White,
        PieceType::King,
    );
    // queenl
    spawn_piece(
        &mut commands,
        game_textures.queenl.clone(),
        (3, 0),
        PieceColor::White,
        PieceType::Queen,
    );
    // rookl
    for i in vec![0, 7] {
        spawn_piece(
            &mut commands,
            game_textures.rookl.clone(),
            (i, 0),
            PieceColor::White,
            PieceType::Rook,
        );
    }
    // bishopl
    for i in vec![2, 5] {
        spawn_piece(
            &mut commands,
            game_textures.bishopl.clone(),
            (i, 0),
            PieceColor::White,
            PieceType::Bishop,
        );
    }
    // knightl
    for i in vec![1, 6] {
        spawn_piece(
            &mut commands,
            game_textures.knightl.clone(),
            (i, 0),
            PieceColor::White,
            PieceType::Knight,
        );
    }
    // pawnl
    for i in 0..8 {
        spawn_piece(
            &mut commands,
            game_textures.pawnl.clone(),
            (i, 1),
            PieceColor::White,
            PieceType::Pawn,
        );
    }
    // kingd
    spawn_piece(
        &mut commands,
        game_textures.kingd.clone(),
        (4, 7),
        PieceColor::White,
        PieceType::King,
    );
    // queend
    spawn_piece(
        &mut commands,
        game_textures.queend.clone(),
        (3, 7),
        PieceColor::Black,
        PieceType::Queen,
    );
    // rookd
    for i in vec![0, 7] {
        spawn_piece(
            &mut commands,
            game_textures.rookd.clone(),
            (i, 7),
            PieceColor::Black,
            PieceType::Rook,
        );
    }
    // bishopd
    for i in vec![2, 5] {
        spawn_piece(
            &mut commands,
            game_textures.bishopd.clone(),
            (i, 7),
            PieceColor::Black,
            PieceType::Bishop,
        );
    }
    // knightd
    for i in vec![1, 6] {
        spawn_piece(
            &mut commands,
            game_textures.knightd.clone(),
            (i, 7),
            PieceColor::Black,
            PieceType::Knight,
        );
    }
    // pawnd
    for i in 0..8 {
        spawn_piece(
            &mut commands,
            game_textures.pawnd.clone(),
            (i, 6),
            PieceColor::Black,
            PieceType::Pawn,
        );
    }
}

fn real_pos(position: (i8, i8)) -> Vec3 {
    return Vec3::new(position.0 as f32 * IMAGE_SIZE.0, position.1 as f32 * IMAGE_SIZE.1, 0.);
}

fn spawn_tile(
    commands: &mut Commands,
    texture: Handle<Image>,
    position: (i8, i8)
) {
    commands.spawn_bundle(SpriteBundle {
        texture,
        transform: Transform {
            translation: real_pos(position),
            ..Default::default()
        },
        ..Default::default()
    }).insert_bundle(PickableBundle::default());
}

fn spawn_piece(
    commands: &mut Commands,
    texture: Handle<Image>,
    position: (i8, i8),
    color: PieceColor,
    piece_type: PieceType,
) {
    commands.spawn_bundle(SpriteBundle {
        texture,
        transform: Transform {
            translation: real_pos(position) + Vec3::new(0., 0., 1.),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Piece {
        color,
        piece_type,
        x: position.0 as i8,
        y: position.1 as i8,
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_startup_system(setup)
        .add_startup_system_to_stage(StartupStage::PostStartup, create_board)
        .add_startup_system_to_stage(StartupStage::PostStartup, create_pieces)
        .run();
}
