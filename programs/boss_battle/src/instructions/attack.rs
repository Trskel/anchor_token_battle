use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Token, Mint};
use anchor_spl::associated_token::AssociatedToken;

use crate::constants::*;
use crate::state::*;
use crate::utils::*;
use crate::events::*;

pub fn handler_attack(ctx: Context<Attack>) -> Result<()> {
    if ctx.accounts.game_data_account.enemy_energy > 0 && ctx.accounts.game_data_account.player_energy > 0 {
        // Get current slot
        let slot = Clock::get()?.slot;
        // Generate pseudo-random number using XORShift with the current slot as seed
        let rand_1 = xorshift64(slot);
        let rand_2 = xorshift64(slot + 5);
        // Calculate random damage
        let random_damage_enemy = rand_1 % (MAX_DAMAGE_ENEMY);
        msg!("Damage to enemy: {}", random_damage_enemy);

        let random_damage_player = rand_2 % (MAX_DAMAGE_PLAYER);
        msg!("Damage to player: {}", random_damage_player);

        // Subtract health from enemy boss, min health is 0
        ctx.accounts.game_data_account.enemy_energy =
            ctx.accounts.game_data_account.enemy_energy.saturating_sub(random_damage_enemy);
        msg!("Enemy Boss Energy: {}", ctx.accounts.game_data_account.enemy_energy);

        // Subtract health from player, min health is 0
        ctx.accounts.game_data_account.player_energy =
            ctx.accounts.game_data_account.player_energy.saturating_sub(random_damage_player);
        msg!("Player Energy: {}", ctx.accounts.game_data_account.player_energy);
    }
    // Check if enemy boss has enough health
    if ctx.accounts.game_data_account.enemy_energy == 0 {
            msg!("Enemy is dead! Claiming GoldenDragon coins.");
            let boss_defeated_event: BossDefeated =  BossDefeated{
                player: ctx.accounts.game_data_account.owner.key(),
                player_energy: ctx.accounts.game_data_account.player_energy,
                timestamp: Clock::get()?.unix_timestamp,
            };
            emit!(boss_defeated_event);
            //Send goldendragons to the player's token account
            let amount = ctx.accounts.treasure_token_account.amount;
            let owner_key = ctx.accounts.game_data_account.owner.key();
            //Seeds for PDA signature of the transaction
            let seeds = &[
                SEED_GAME,
                owner_key.as_ref(),
                &[ctx.bumps.game_data_account],
            ];
            let signer_seeds = &[&seeds[..]];

            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer{
                    from: ctx.accounts.treasure_token_account.to_account_info(),
                    to: ctx.accounts.player_token_account.to_account_info(),
                    authority: ctx.accounts.game_data_account.to_account_info(), //The PDA
                },
                signer_seeds,
            );
            token::transfer(cpi_ctx, amount)?;
        } else if ctx.accounts.game_data_account.player_energy == 0 { 
            //player dead before killing the enemy
            msg!("You are dead! you loose half of your GoldenDragon coins");
            let amount_lost = ctx.accounts.player_token_account.amount / 2;
            let cpi_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.player_token_account.to_account_info(),
                    to: ctx.accounts.treasure_token_account.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            );
            token::transfer(cpi_ctx, amount_lost)?;
        }


        Ok(())
    }

#[derive(Accounts)]
pub struct Attack<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds=[SEED_GAME, owner.key().as_ref()],
        bump,
        constraint = game_data_account.owner == owner.key(),
    )]
    pub game_data_account: Account<'info, GameData>,

    ///Mint account to create the goldendragon tokens
    pub goldendragon_mint: Account<'info, Mint>,

    ///ATA to hold the player's tokens
    #[account(
        mut,
        //associated_token::mint = goldendragon_mint,
        //associated_token::authority = owner,
    )]
    pub player_token_account: Account<'info, TokenAccount>,

    ///ATA to hold the treasure (enemy's) tokens
    #[account(
        mut,
        //associated_token::mint = goldendragon_mint,
        //associated_token::authority = game_data_account,
    )]
    pub treasure_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
