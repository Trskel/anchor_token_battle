use anchor_lang::prelude::*;

#[constant]
pub const SEED_GAME: &[u8] = b"Boss";
pub const MAX_ENERGY_PLAYER: u64 = 100;
pub const MAX_ENERGY_ENEMY: u64 = 100;
pub const MAX_DAMAGE_PLAYER: u64 = 30;
pub const MAX_DAMAGE_ENEMY: u64 = 50;
pub const INITIAL_PLAYER_TOKENS: u64 = 10;
pub const INITIAL_ENEMY_TOKENS: u64 = 5;

