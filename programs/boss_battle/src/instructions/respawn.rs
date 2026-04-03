use anchor_lang::prelude::*;

use crate::state::*;
use crate::constants::*;

pub fn handler_respawn(ctx: Context<Respawn>) -> Result<()> {
    ctx.accounts.game_data_account.player_energy = MAX_ENERGY_PLAYER;
    ctx.accounts.game_data_account.enemy_energy = MAX_ENERGY_ENEMY;
    Ok(())
}

#[derive(Accounts)]
pub struct Respawn<'info>{
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_GAME, owner.key().as_ref()],
        bump,
        constraint = game_data_account.owner == owner.key(),
    )]
    pub game_data_account: Account<'info, GameData>,

}