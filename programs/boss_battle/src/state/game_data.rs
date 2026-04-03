use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct GameData {
    pub owner: Pubkey,
    pub enemy_energy: u64,
    pub player_energy: u64,
}