use bevy::prelude::*;

pub const SCALING_FACTOR: f32 = 1.5;
pub const IMAGE_SIZE: (f32, f32) = (SCALING_FACTOR * 45., SCALING_FACTOR * 45.);
pub const TILEL_SPRITE: &str = "sprites/tilel.png";
pub const KINGL_SPRITE: &str = "sprites/kl.png";
pub const QUEENL_SPRITE: &str = "sprites/ql.png";
pub const ROOKL_SPRITE: &str = "sprites/rl.png";
pub const BISHOPL_SPRITE: &str = "sprites/bl.png";
pub const KNIGHTL_SPRITE: &str = "sprites/nl.png";
pub const PAWNL_SPRITE: &str = "sprites/pl.png";
pub const TILED_SPRITE: &str = "sprites/tiled.png";
pub const KINGD_SPRITE: &str = "sprites/kd.png";
pub const QUEEND_SPRITE: &str = "sprites/qd.png";
pub const ROOKD_SPRITE: &str = "sprites/rd.png";
pub const BISHOPD_SPRITE: &str = "sprites/bd.png";
pub const KNIGHTD_SPRITE: &str = "sprites/nd.png";
pub const PAWND_SPRITE: &str = "sprites/pd.png";
pub const HIGHLIGHT_SPRITE: &str = "sprites/highlight.png";

pub struct GameTextures {
    pub tilel: Handle<Image>,
    pub kingl: Handle<Image>,
    pub queenl: Handle<Image>,
    pub rookl: Handle<Image>,
    pub bishopl: Handle<Image>,
    pub knightl: Handle<Image>,
    pub pawnl: Handle<Image>,
    pub tiled: Handle<Image>,
    pub kingd: Handle<Image>,
    pub queend: Handle<Image>,
    pub rookd: Handle<Image>,
    pub bishopd: Handle<Image>,
    pub knightd: Handle<Image>,
    pub pawnd: Handle<Image>,
    pub highlight: Handle<Image>,
}
