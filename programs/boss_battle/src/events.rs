use anchor_lang::prelude::*;

#[event]
pub struct BossDefeated {
    pub player: Pubkey,
    pub player_energy: u64,
    pub timestamp: i64,
}