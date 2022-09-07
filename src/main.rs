use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

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

#[derive(Clone, Copy, PartialEq, Component)]
struct Position(i8, i8);

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
	commands
        .insert_resource(game_textures);

	commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    commands
        .insert_resource(MousePosition { position: None });

    commands
        .insert_resource(SelectedSquare { position: None });
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
				Position(i, j)
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
		Position(4, 0),
		PieceColor::White,
		PieceType::King,
	);
	// queenl
	spawn_piece(
		&mut commands,
		game_textures.queenl.clone(),
		Position(3, 0),
		PieceColor::White,
		PieceType::Queen,
	);
	// rookl
	for i in &[0, 7] {
		spawn_piece(
			&mut commands,
			game_textures.rookl.clone(),
			Position(*i, 0),
			PieceColor::White,
			PieceType::Rook,
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
		);
	}
	// kingd
	spawn_piece(
		&mut commands,
		game_textures.kingd.clone(),
		Position(4, 7),
		PieceColor::White,
		PieceType::King,
	);
	// queend
	spawn_piece(
		&mut commands,
		game_textures.queend.clone(),
		Position(3, 7),
		PieceColor::Black,
		PieceType::Queen,
	);
	// rookd
	for i in &[0, 7] {
		spawn_piece(
			&mut commands,
			game_textures.rookd.clone(),
			Position(*i, 7),
			PieceColor::Black,
			PieceType::Rook,
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
		);
	}
}

fn real_pos(position: Position) -> Vec3 {
	Vec3::new((position.0 as f32 - 3.5) * IMAGE_SIZE.0,
                (position.1 as f32 - 3.5) * IMAGE_SIZE.1,
                0.)
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
    }
    else {
        Some(Position(x as i8, y as i8))
    }
}

fn spawn_tile(
	commands: &mut Commands,
	texture: Handle<Image>,
	position: Position
) {
    let mut transform = Transform {
			translation: real_pos(position),
			..Default::default()
    };
    transform.scale *= SCALING_FACTOR;

	commands.spawn_bundle(SpriteBundle {
		texture,
		transform,
		..Default::default()
	});
}

fn spawn_piece(
	commands: &mut Commands,
	texture: Handle<Image>,
	position: Position,
	color: PieceColor,
	piece_type: PieceType,
) {
    let mut transform = Transform {
			translation: real_piece_pos(position),
			..Default::default()
    };
    transform.scale *= SCALING_FACTOR;

	commands.spawn_bundle(SpriteBundle {
		texture,
		transform,
		..Default::default()
    }).insert(position);

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
		color,
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

fn mouse_pressed_system(
    buttons: Res<Input<MouseButton>>,
    mpos: Res<MousePosition>,
    mut sel: ResMut<SelectedSquare>,
    query: Query<(Entity, &mut Position, &mut Transform)>,
    commands: Commands,
) {
    if buttons.just_pressed(MouseButton::Left) {
        eprintln!("huray!");
        if let Some(position) = mpos.position {
            if let Some(sel_pos) = sel.position { // a move from sel_pos to position
                eprintln!("a move is happening from ({}, {}) to ({}, {})",
                sel_pos.0,
                sel_pos.1,
                position.0,
                position.1
                );

                // TODO dać tu jakąś funkcję GameState a wywołanie tej funkcji dać do GameState
                move_piece(commands, query, sel_pos, position);

                sel.position = None;
            }
            else {
                sel.position = mpos.position;
            }
        }
        else {
            sel.position = None;
        }
        if let Some(position) = sel.position {
            eprintln!("Some({}, {})", position.0, position.1);
        }
        else {
            eprintln!("None");
        }
    }
}

fn delete_piece(
    mut commands: Commands,
    query: &mut Query<(Entity, &mut Position, &mut Transform)>,
    position: Position,
) {
    for (entity, piece_position, mut _transform) in query.iter_mut() {
        if *piece_position != position {
            continue;
        }
        eprintln!("papa :(");
        commands.entity(entity).despawn();
    }
}

fn move_piece(
    commands: Commands,
    mut query: Query<(Entity, &mut Position, &mut Transform)>,
    from: Position,
    to: Position,
) {
    delete_piece(commands, &mut query, to);
    for (mut _entity, mut piece_position, mut transform) in query.iter_mut() {
        if *piece_position != from {
            continue;
        }
        eprintln!("hejo!");
        *piece_position = to;
        transform.translation = real_piece_pos(to);
    }
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup)
		.add_startup_system_to_stage(StartupStage::PostStartup, create_board)
		.add_startup_system_to_stage(StartupStage::PostStartup, create_pieces)
        .add_system(cursor_position_system)
        .add_system(mouse_pressed_system)
		.run();
}
