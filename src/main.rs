use bevy::prelude::*;

const IMAGE_SIZE: (f32, f32) = (45., 45.);

fn create_board(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: Res<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let mut window = windows.get_primary_mut().unwrap();
    window.set_position(IVec2::new(1000, 1000));

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("sprites/pl.png"),
        ..Default::default()
    });

    let tilel = asset_server.load("sprites/tilel.png");
    let tiled = asset_server.load("sprites/tiled.png");

    for i in 0..8 {
        for j in 0..8 {
            let tile = if (i + j) % 2 == 0 { &tilel } else { &tiled };
            spawn_tile(&mut commands, tile.clone(), Vec3::new(i as f32 * IMAGE_SIZE.0, j as f32 * IMAGE_SIZE.1, 10.));
        }
    }
}

fn spawn_tile(
    commands: &mut Commands,
    texture: Handle<Image>,
    position: Vec3,
) {
    commands.spawn_bundle(SpriteBundle {
        texture,
        transform: Transform {
            translation: position,
            ..Default::default()
        },
        ..Default::default()
    });
}

/*fn spawn_pawn(
    commands: &mut Commands,
    color: Color,
    pawn_handle: Handle<Image>,
    position: Vec2,
    ) {
    let pawn_image = asset_server.load("sprites/pl.svg");
    /*commands
        .spawn_bundle(SpriteBundle {
            sprite: 
            */
}*/

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(create_board)
        .run();
}
