use std::collections::HashMap;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

const INFINITY: f32 = 1000000.;
const SCALING_FACTOR: f32 = 1.5;
const IMAGE_SIZE: (f32, f32) = (SCALING_FACTOR * 45., SCALING_FACTOR * 45.);

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
const HIGHLIGHT_SPRITE: &str = "sprites/highlight.png";

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
    highlight: Handle<Image>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Debug, Clone, Copy, PartialEq, Component)]
struct Position(i8, i8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
struct Piece {
    piece_color: PieceColor,
    piece_type: PieceType,
    x: i8,
    y: i8,
}

impl Piece {
    // TODO work on this function, maybe add position as an argument and maybe some other variables
    fn value(&self) -> f32 {
        let value = match self.piece_type {
            PieceType::King => INFINITY,
            PieceType::Queen => 9.,
            PieceType::Rook => 5.,
            PieceType::Bishop => 3.,
            PieceType::Knight => 3.,
            PieceType::Pawn => 1.,
        };
        match self.piece_color {
            PieceColor::White => value,
            PieceColor::Black => -value,
        }
    }
    fn can_move_king(&self, x: i8, y: i8) -> bool {
        i8::abs(self.x - x) <= 1 && i8::abs(self.y - y) <= 1
    }

    fn can_move_queen(&self, x: i8, y: i8) -> bool {
        self.can_move_rook(x, y) || self.can_move_bishop(x, y)
    }

    fn can_move_rook(&self, x: i8, y: i8) -> bool {
        self.x == x || self.y == y
    }

    fn can_move_bishop(&self, x: i8, y: i8) -> bool {
        i8::abs(self.x - x) == i8::abs(self.y - y)
    }

    fn can_move_knight(&self, x: i8, y: i8) -> bool {
        (i8::abs(self.x - x) == 1 && i8::abs(self.y - y) == 2)
            || (i8::abs(self.x - x) == 2 && i8::abs(self.y - y) == 1)
    }

    fn can_move_pawn(&self, x: i8, y: i8) -> bool {
        true
    }

    fn can_move(&self, x: i8, y: i8) -> bool {
        // ignores attacks on the king and pins
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

#[derive(Clone, PartialEq, Eq, Hash)]
struct GameState {
    board: [[Option<Piece>; 8]; 8], // może zmienić na HashSet indeksowany Position
    now_moves: PieceColor,
    player_moves: bool,
}

struct Move(Position, Position);

impl GameState {
    // TODO work on this function, add alfa-beta later
    fn evaluate(&self, level: u32, cache: &mut HashMap<(GameState, u32), f32>) -> f32 {
        let key = (self.clone(), level);
        if let Some(x) = cache.get(&key) {
            *x
        }
        else {
            let value = if level > 0 {
                let mut score = match self.now_moves {
                    PieceColor::White => -INFINITY,
                    PieceColor::Black => INFINITY,
                };
                for Move(from, to) in self.gen_legal_moves() {
                    println!("(level, from, to) = ({:?}, {:?}, {:?})", level, from, to);
                    let mut next_state = self.clone();
                    next_state.move_piece(from, to);
                    let next_state_score = next_state.evaluate(level - 1, cache);
                    match self.now_moves {
                        PieceColor::White => score = score.max(next_state_score),
                        PieceColor::Black => score = score.min(next_state_score),
                    };
                }
                score
            }
            else {
                let mut score = 0.;
                for i in 0..8 {
                    for j in 0..8 {
                        if let Some(x) = self.board[i][j] {
                            score += x.value();
                            println!("ss: {}", score)
                        }
                    }
                }
                score
            };
            cache.insert(key, value);
            value
        }
    }
    fn gen_legal_moves(&self) -> Vec<Move> {
        vec![]
    }
    fn move_piece(&mut self,
                  from: Position,
                  to: Position,
    ) {
        let piece = self.board[from.0 as usize][from.1 as usize].take();
        assert!(piece.is_some());
        let mut piece = piece.unwrap();
        piece.move_piece(to.0, to.1);
        self.board[to.0 as usize][to.1 as usize] = Some(piece);
        self.now_moves = match self.now_moves {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        };
    }
    fn move_piece_for_real(
        &mut self,
        commands: &mut Commands,
        query: Query<(Entity, &mut Position, &mut Transform)>,
        from: Position,
        to: Position,
    ) {
        self.move_piece(from, to);
        move_piece_physically(commands, query, from, to);
        self.player_moves = !self.player_moves;
    }
    fn player_move(
        &mut self,
        commands: &mut Commands,
        query: Query<(Entity, &mut Position, &mut Transform)>,
        from: Position,
        to: Position,
    ) {
        if !self.player_moves {
            return;
        }

        self.move_piece_for_real(commands, query, from, to);
    }
    // TODO add sth to this function or delete it
    fn computer_move(
        &mut self,
        commands: &mut Commands,
        query: Query<(Entity, &mut Position, &mut Transform)>,
        from: Position,
        to: Position,
    ) {
        self.move_piece_for_real(commands, query, from, to);
    }
    // TODO add more functions
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    commands.insert_resource(MousePosition { position: None });

    commands.insert_resource(SelectedSquare { position: None });

    commands.insert_resource(GameState {
        board: [[None; 8]; 8],
        now_moves: PieceColor::White,
        player_moves: rand::random::<bool>(),
    });
}

fn create_board(mut commands: Commands, game_textures: Res<GameTextures>) {
    let tilel = game_textures.tilel.clone();
    let tiled = game_textures.tiled.clone();
    for i in 0..8 {
        for j in 0..8 {
            let tile = if (i + j) % 2 == 0 { &tiled } else { &tilel };
            spawn_tile(&mut commands, tile.clone(), Position(i, j), false);
        }
    }
}

fn create_pieces(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut game_state: ResMut<GameState>,
) {
    // kingl
    spawn_piece(
        &mut commands,
        game_textures.kingl.clone(),
        Position(4, 0),
        PieceColor::White,
        PieceType::King,
        &mut game_state,
    );
    // queenl
    spawn_piece(
        &mut commands,
        game_textures.queenl.clone(),
        Position(3, 0),
        PieceColor::White,
        PieceType::Queen,
        &mut game_state,
    );
    // rookl
    for i in &[0, 7] {
        spawn_piece(
            &mut commands,
            game_textures.rookl.clone(),
            Position(*i, 0),
            PieceColor::White,
            PieceType::Rook,
            &mut game_state,
        );
    }
    // bishopl
    for i in &[2, 5] {
        spawn_piece(
            &mut commands,
            game_textures.bishopl.clone(),
            Position(*i, 0),
            PieceColor::White,
            PieceType::Bishop,
            &mut game_state,
        );
    }
    // knightl
    for i in &[1, 6] {
        spawn_piece(
            &mut commands,
            game_textures.knightl.clone(),
            Position(*i, 0),
            PieceColor::White,
            PieceType::Knight,
            &mut game_state,
        );
    }
    // pawnl
    for i in 0..8 {
        spawn_piece(
            &mut commands,
            game_textures.pawnl.clone(),
            Position(i, 1),
            PieceColor::White,
            PieceType::Pawn,
            &mut game_state,
        );
    }
    // kingd
    spawn_piece(
        &mut commands,
        game_textures.kingd.clone(),
        Position(4, 7),
        PieceColor::Black,
        PieceType::King,
        &mut game_state,
    );
    // queend
    spawn_piece(
        &mut commands,
        game_textures.queend.clone(),
        Position(3, 7),
        PieceColor::Black,
        PieceType::Queen,
        &mut game_state,
    );
    // rookd
    for i in &[0, 7] {
        spawn_piece(
            &mut commands,
            game_textures.rookd.clone(),
            Position(*i, 7),
            PieceColor::Black,
            PieceType::Rook,
            &mut game_state,
        );
    }
    // bishopd
    for i in &[2, 5] {
        spawn_piece(
            &mut commands,
            game_textures.bishopd.clone(),
            Position(*i, 7),
            PieceColor::Black,
            PieceType::Bishop,
            &mut game_state,
        );
    }
    // knightd
    for i in &[1, 6] {
        spawn_piece(
            &mut commands,
            game_textures.knightd.clone(),
            Position(*i, 7),
            PieceColor::Black,
            PieceType::Knight,
            &mut game_state,
        );
    }
    // pawnd
    for i in 0..8 {
        spawn_piece(
            &mut commands,
            game_textures.pawnd.clone(),
            Position(i, 6),
            PieceColor::Black,
            PieceType::Pawn,
            &mut game_state,
        );
    }
}

fn real_pos(position: Position) -> Vec3 {
    Vec3::new(
        (position.0 as f32 - 3.5) * IMAGE_SIZE.0,
        (position.1 as f32 - 3.5) * IMAGE_SIZE.1,
        0.,
    )
}

fn real_piece_pos(position: Position) -> Vec3 {
    real_pos(position) + Vec3::new(0., 0., 1.)
}

fn game_pos(vec: Vec3) -> Option<Position> {
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
struct Highlight {}

fn spawn_tile(
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

fn spawn_piece(
    commands: &mut Commands,
    texture: Handle<Image>,
    position: Position,
    piece_color: PieceColor,
    piece_type: PieceType,
    game_state: &mut ResMut<GameState>,
) {
    let mut transform = Transform {
        translation: real_piece_pos(position),
        ..Default::default()
    };
    transform.scale *= SCALING_FACTOR;

    commands
        .spawn_bundle(SpriteBundle {
            texture,
            transform,
            ..Default::default()
        })
        .insert(position);

    game_state.board[position.0 as usize][position.1 as usize] = Some(Piece {
        piece_color,
        piece_type,
        x: position.0,
        y: position.1,
    });
    /*

    commands.spawn_bundle(SpriteBundle {
        texture,
        transform: Transform {
            translation: real_piece_pos(position),
            ..Default::default()
        },
        ..Default::default()
    }).insert(position);

    */

    /*
    }).insert(Piece {
        piece_color,
        piece_type,
        x: position.0,
        y: position.1,
    });
    */
}

#[derive(Component)]
struct MainCamera;

struct MousePosition {
    position: Option<Position>,
}

fn cursor_position_system(
    wnds: Res<Windows>,
    mut mpos: ResMut<MousePosition>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();

    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    if let Some(screen_pos) = wnd.cursor_position() {
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        mpos.position = game_pos(world_pos);
    }
}

struct SelectedSquare {
    position: Option<Position>,
}

fn delete_highlight(commands: &mut Commands, query: &Query<Entity, With<Highlight>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn mouse_pressed_system(
    buttons: Res<Input<MouseButton>>,
    mpos: Res<MousePosition>,
    mut sel: ResMut<SelectedSquare>,
    query: Query<(Entity, &mut Position, &mut Transform)>,
    query_highlight: Query<Entity, With<Highlight>>,
    game_textures: Res<GameTextures>,
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(position) = mpos.position {
            if let Some(sel_pos) = sel.position {
                game_state.player_move(&mut commands, query, sel_pos, position);
                sel.position = None;
                delete_highlight(&mut commands, &query_highlight);
            } else {
                sel.position = mpos.position;
                spawn_tile(
                    &mut commands,
                    game_textures.highlight.clone(),
                    position,
                    true,
                );
            }
        } else {
            sel.position = None;
            delete_highlight(&mut commands, &query_highlight);
        }
    }
    if buttons.just_pressed(MouseButton::Right) {
        sel.position = None;
        delete_highlight(&mut commands, &query_highlight);
    }
}

fn delete_piece_physically(
    commands: &mut Commands,
    query: &mut Query<(Entity, &mut Position, &mut Transform)>,
    position: Position,
) {
    for (entity, piece_position, mut _transform) in query.iter_mut() {
        if *piece_position != position {
            continue;
        }
        commands.entity(entity).despawn();
    }
}

fn move_piece_physically(
    commands: &mut Commands,
    mut query: Query<(Entity, &mut Position, &mut Transform)>,
    from: Position,
    to: Position,
) {
    delete_piece_physically(commands, &mut query, to);
    for (mut _entity, mut piece_position, mut transform) in query.iter_mut() {
        if *piece_position != from {
            continue;
        }
        *piece_position = to;
        transform.translation = real_piece_pos(to);
    }
}

fn computer_moves_system(
    mut game_state: ResMut<GameState>,
) {
    if game_state.player_moves {
        return;
    }
    let mut cache = HashMap::<(GameState, u32), f32>::new();
    let score = game_state.evaluate(3, &mut cache);
    let score2 = game_state.evaluate(0, &mut cache);
    println!("Score: {}", score);
    println!("Score2: {}", score2);

    eprintln!("Thinking ...");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system_to_stage(StartupStage::PostStartup, create_board)
        .add_startup_system_to_stage(StartupStage::PostStartup, create_pieces)
        .add_system(cursor_position_system)
        .add_system(mouse_pressed_system)
        .add_system(computer_moves_system)
        .run();
}
