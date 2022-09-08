use bevy::ecs::event::Events;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use rand::seq::SliceRandom;
use std::collections::HashMap;

const TABLE_QUEEN: [[i32; 8]; 8] = [
    [-20, -10, -10, -5, -5, -10, -10, -20],
    [-10, 0, 0, 0, 0, 0, 0, -10],
    [-10, 0, 5, 5, 5, 5, 0, -10],
    [-5, 0, 5, 5, 5, 5, 0, -5],
    [0, 0, 5, 5, 5, 5, 0, -5],
    [-10, 5, 5, 5, 5, 5, 0, -10],
    [-10, 0, 5, 0, 0, 0, 0, -10],
    [-20, -10, -10, -5, -5, -10, -10, -20],
];
const TABLE_ROOK: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [5, 10, 10, 10, 10, 10, 10, 5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [0, 0, 0, 5, 5, 0, 0, 0],
];
const TABLE_BISHOP: [[i32; 8]; 8] = [
    [-20, -10, -10, -10, -10, -10, -10, -20],
    [-10, 0, 0, 0, 0, 0, 0, -10],
    [-10, 0, 5, 10, 10, 5, 0, -10],
    [-10, 5, 5, 10, 10, 5, 5, -10],
    [-10, 0, 10, 10, 10, 10, 0, -10],
    [-10, 10, 10, 10, 10, 10, 10, -10],
    [-10, 5, 0, 0, 0, 0, 5, -10],
    [-20, -10, -10, -10, -10, -10, -10, -20],
];
const TABLE_KNIGHT: [[i32; 8]; 8] = [
    [-50, -40, -30, -30, -30, -30, -40, -50],
    [-40, -20, 0, 0, 0, 0, -20, -40],
    [-30, 0, 10, 15, 15, 10, 0, -30],
    [-30, 5, 15, 20, 20, 15, 5, -30],
    [-30, 0, 15, 20, 20, 15, 0, -30],
    [-30, 5, 10, 15, 15, 10, 5, -30],
    [-40, -20, 0, 5, 5, 0, -20, -40],
    [-50, -40, -30, -30, -30, -30, -40, -50],
];
const TABLE_PAWN: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [50, 50, 50, 50, 50, 50, 50, 50],
    [10, 10, 20, 30, 30, 20, 10, 10],
    [5, 5, 10, 25, 25, 10, 5, 5],
    [0, 0, 0, 20, 20, 0, 0, 0],
    [5, -5, -10, 0, 0, -10, -5, 5],
    [5, 10, 10, -20, -20, 10, 10, 5],
    [0, 0, 0, 0, 0, 0, 0, 0],
];

const INFINITY: f32 = 1000000.;
const BIG_INFINITY: f32 = 10. * INFINITY;
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
    fn table_position(&self) -> Position {
        match self.piece_color {
            PieceColor::White => Position(7 - self.y, self.x),
            PieceColor::Black => Position(self.y, self.x),
        }
    }
    fn value(&self) -> f32 {
        let Position(x, y) = self.table_position();
        let (x, y) = (x as usize, y as usize);
        let value = match self.piece_type {
            PieceType::King => INFINITY, // + TABLE_KING[x][y] as f32,
            PieceType::Queen => 900. + TABLE_QUEEN[x][y] as f32,
            PieceType::Rook => 500. + TABLE_ROOK[x][y] as f32,
            PieceType::Bishop => 330. + TABLE_BISHOP[x][y] as f32,
            PieceType::Knight => 320. + TABLE_KNIGHT[x][y] as f32,
            PieceType::Pawn => 100. + TABLE_PAWN[x][y] as f32,
        };
        match self.piece_color {
            PieceColor::White => value,
            PieceColor::Black => -value,
        }
    }
    fn is_legal(&self, x: i8, y: i8, board: Board) -> bool {
        if !(0..8).contains(&x) || !(0..8).contains(&y) {
            return false;
        }
        if let Some(piece) = board[x as usize][y as usize] {
            self.piece_color != piece.piece_color
        } else {
            true
        }
    }
    fn is_capture(&self, x: i8, y: i8, board: Board) -> bool {
        board[x as usize][y as usize].is_some()
    }
    fn gen_consecutive(&self, board: Board, diffs: Vec<(i8, i8)>) -> Vec<Move> {
        let mut legal_moves = Vec::new();
        for (dx, dy) in diffs {
            let mut x = self.x + dx;
            let mut y = self.y + dy;
            while self.is_legal(x, y, board) {
                legal_moves.push((Position(self.x, self.y), Position(x, y)));
                if self.is_capture(x, y, board) {
                    break;
                }
                x += dx;
                y += dy;
            }
        }
        legal_moves
    }
    fn gen_rook(&self, board: Board) -> Vec<Move> {
        self.gen_consecutive(board, vec![(1, 0), (-1, 0), (0, 1), (0, -1)])
    }
    fn gen_bishop(&self, board: Board) -> Vec<Move> {
        self.gen_consecutive(board, vec![(1, 1), (1, -1), (-1, 1), (-1, -1)])
    }
    fn gen_legal_moves(&self, board: Board) -> Vec<Move> {
        let mut legal_moves: Vec<Move> = Vec::new();

        match self.piece_type {
            PieceType::King => {
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if self.is_legal(self.x + dx, self.y + dy, board) {
                            legal_moves.push((
                                Position(self.x, self.y),
                                Position(self.x + dx, self.y + dy),
                            ));
                        }
                    }
                }
            }
            PieceType::Queen => {
                legal_moves.append(&mut self.gen_rook(board));
                legal_moves.append(&mut self.gen_bishop(board));
            }
            PieceType::Rook => {
                legal_moves.append(&mut self.gen_rook(board));
            }
            PieceType::Bishop => {
                legal_moves.append(&mut self.gen_bishop(board));
            }
            PieceType::Knight => {
                for dx in -2..=2 {
                    for dy in -2..=2 {
                        if i8::abs(dx * dy) == 2 && self.is_legal(self.x + dx, self.y + dy, board) {
                            legal_moves.push((
                                Position(self.x, self.y),
                                Position(self.x + dx, self.y + dy),
                            ));
                        }
                    }
                }
            }
            PieceType::Pawn => {
                let dir = match self.piece_color {
                    PieceColor::White => 1,
                    PieceColor::Black => -1,
                };
                for dx in &[-1, 1] {
                    if self.is_legal(self.x + dx, self.y + dir, board)
                        && self.is_capture(self.x + dx, self.y + dir, board)
                    {
                        legal_moves.push((
                            Position(self.x, self.y),
                            Position(self.x + dx, self.y + dir),
                        ));
                    }
                }
                if self.is_legal(self.x, self.y + dir, board)
                    && !self.is_capture(self.x, self.y + dir, board)
                {
                    legal_moves.push((Position(self.x, self.y), Position(self.x, self.y + dir)));
                    if self.y == (-25 * dir + 35) / 10
                        && self.is_legal(self.x, self.y + 2 * dir, board)
                        && !self.is_capture(self.x, self.y + 2 * dir, board)
                    {
                        legal_moves
                            .push((Position(self.x, self.y), Position(self.x, self.y + 2 * dir)));
                    }
                }
            }
        }
        legal_moves
    }
    fn move_piece(&mut self, x: i8, y: i8) {
        self.x = x;
        self.y = y;
    }
}

type Board = [[Option<Piece>; 8]; 8];

#[derive(Clone, PartialEq, Eq, Hash)]
struct GameState {
    board: Board, // może zmienić na HashSet indeksowany Position
    now_moves: PieceColor,
    player_moves: bool,
}

type Move = (Position, Position);
//struct Move(Position, Position);

impl GameState {
    fn kings(&self) -> (bool, bool) {
        let mut white_king = false;
        let mut black_king = false;
        for i in 0..8 {
            for j in 0..8 {
                if let Some(piece) = self.board[i][j] {
                    if piece.piece_type == PieceType::King {
                        if piece.piece_color == PieceColor::White {
                            white_king = true;
                        } else {
                            black_king = true;
                        }
                    }
                }
            }
        }
        (white_king, black_king)
    }
    // TODO work on this function
    fn evaluate(
        &self,
        level: i32,
        cache: &mut HashMap<(GameState, i32), f32>,
        mut alpha: f32,
        mut beta: f32,
    ) -> f32 {
        let key = (self.clone(), level);
        if let Some(x) = cache.get(&key) {
            *x
        } else {
            let (white_king, black_king) = self.kings();
            if !white_king {
                let value = -2. * INFINITY - level as f32 / 10.;
                cache.insert(key, value);
                return value;
            }
            if !black_king {
                let value = 2. * INFINITY + level as f32 / 10.;
                cache.insert(key, value);
                return value;
            }
            /*
            let mut legal_moves = self.gen_legal_moves;
            let captures = legal_moves
                .iter()
                .filter(
            */
            let value = if level > 0 {
                let mut score = match self.now_moves {
                    PieceColor::White => -BIG_INFINITY,
                    PieceColor::Black => BIG_INFINITY,
                };
                for (from, to) in self.gen_legal_moves() {
                    let mut next_state = self.clone();
                    next_state.move_piece(from, to);
                    let next_state_score = next_state.evaluate(level - 1, cache, alpha, beta);
                    match self.now_moves {
                        PieceColor::White => {
                            score = score.max(next_state_score);
                            if score > beta {
                                break;
                            }
                            alpha = alpha.max(score);
                        }
                        PieceColor::Black => {
                            score = score.min(next_state_score);
                            if score < alpha {
                                break;
                            }
                            beta = beta.min(score);
                        }
                    };
                }
                score
            } else {
                let mut score = 0.;
                for i in 0..8 {
                    for j in 0..8 {
                        if let Some(x) = self.board[i][j] {
                            score += x.value();
                        }
                    }
                }
                score
            };
            cache.insert(key, value);
            value
        }
    }
    fn is_capture(&self, _from: Position, to: Position) -> bool {
        self.board[to.0 as usize][to.1 as usize].is_some()
    }
    fn gen_legal_moves(&self) -> Vec<Move> {
        let mut legal_moves = Vec::new();
        for i in 0..8 {
            for j in 0..8 {
                if let Some(piece) = self.board[i][j] {
                    if piece.piece_color == self.now_moves {
                        legal_moves.append(&mut piece.gen_legal_moves(self.board));
                    }
                }
            }
        }
        legal_moves.sort_unstable_by(|(afrom, ato), (bfrom, bto)| {
            self.is_capture(*bfrom, *bto)
                .cmp(&self.is_capture(*afrom, *ato))
        });
        legal_moves
    }
    fn move_piece(&mut self, from: Position, to: Position) {
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
        if !self.player_moves || !self.gen_legal_moves().contains(&(from, to)) {
            return;
        }
        self.move_piece_for_real(commands, query, from, to);
    }
    fn computer_move(
        &mut self,
        commands: &mut Commands,
        query: Query<(Entity, &mut Position, &mut Transform)>,
        from: Position,
        to: Position,
    ) {
        assert!(!self.player_moves && self.gen_legal_moves().contains(&(from, to)));
        self.move_piece_for_real(commands, query, from, to);
    }
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

    let player_moves = rand::random::<bool>();
    if player_moves {
        println!("Your move");
    }
    commands.insert_resource(GameState {
        board: [[None; 8]; 8],
        now_moves: PieceColor::White,
        player_moves,
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
    mut commands: Commands,
    query: Query<(Entity, &mut Position, &mut Transform)>,
    mut game_state: ResMut<GameState>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    let (white_king, black_king) = game_state.kings();
    if !white_king {
        println!("Black wins!");
        std::thread::sleep(std::time::Duration::from_millis(1000));
        app_exit_events.send(bevy::app::AppExit);
    }
    if !black_king {
        println!("White wins!");
        std::thread::sleep(std::time::Duration::from_millis(1000));
        app_exit_events.send(bevy::app::AppExit);
    }
    if game_state.player_moves {
        return;
    }

    println!("Thinking ...");
    let mut cache = HashMap::<(GameState, i32), f32>::new();
    let depth = 5;
    let score = game_state.evaluate(depth, &mut cache, -BIG_INFINITY, BIG_INFINITY);
    let possible_moves = game_state.gen_legal_moves();
    let good_moves = possible_moves
        .into_iter()
        .filter(|(from, to)| {
            let mut next_state = game_state.clone();
            next_state.move_piece(*from, *to);
            let next_state_score =
                next_state.evaluate(depth - 1, &mut cache, -BIG_INFINITY, BIG_INFINITY);
            score == next_state_score
        })
        .collect::<Vec<Move>>();
    println!("good moves: {:?}", good_moves);

    let computer_move = good_moves.choose(&mut rand::thread_rng()).unwrap();

    game_state.computer_move(&mut commands, query, computer_move.0, computer_move.1);
    println!("Your move");
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
