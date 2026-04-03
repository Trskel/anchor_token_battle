use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, MintTo};
use anchor_spl::associated_token::*;
use crate::constants::*;
use crate::state::*;

/// Handles the creation and initial state setup of a `GameData` account.
/// This is a one-time operation per player-program pair.
/// It also creates the goldendragon tokens mint and ATAs (empty for now)
pub fn handler_initialize(ctx: Context<Initialize>) -> Result<()> {
    msg!("Initializing program for: {:?}", ctx.program_id);
    
    // Mutably borrow the game data account
    let game_data = &mut ctx.accounts.game_data_account;
    game_data.owner = ctx.accounts.owner.key();    

    // Energy starts at the default values.
    game_data.player_energy = MAX_ENERGY_PLAYER;
    game_data.enemy_energy = MAX_ENERGY_ENEMY;

 // 1. Mint al Jugador
    token::mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.goldendragon_mint.to_account_info(),
                to: ctx.accounts.player_token_account.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(), // Asumiendo que el owner es la autoridad
            },
        ),
        INITIAL_PLAYER_TOKENS,
    )?;

    // 2. Mint al Tesoro del Enemigo (Bóveda)
    token::mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.goldendragon_mint.to_account_info(),
                to: ctx.accounts.treasure_token_account.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        INITIAL_ENEMY_TOKENS,
    )?;
    Ok(())
}

/// Accounts required to initialize the game state.
#[derive(Accounts)]
pub struct Initialize<'info> {
    /// The player initializing the game. This account signs the transaction 
    /// and pays for the account creation (lamports for rent-exemption).
    #[account(mut)]
    pub owner: Signer<'info>,

    /// The Program Derived Address (PDA) that stores the player's game state.
    /// - Seeds: "game" (prefix) + owner's public key.
    /// - Space: Calculated automatically via `GameData::INIT_SPACE`.
    /// - Bump: Stored by Anchor to ensure the PDA can sign for itself later.
    #[account(
        init,
        payer = owner,
        seeds = [SEED_GAME, owner.key().as_ref()],
        bump,
        space = 8 + GameData::INIT_SPACE
    )]
    pub game_data_account: Account<'info, GameData>,

    ///Mint account to create the goldendragon tokens
    #[account(mut)]
    pub goldendragon_mint: Account<'info, anchor_spl::token::Mint>,

    ///Associated Token Account for the player to store their tokens
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = goldendragon_mint,
        associated_token::authority = owner,
    )]
    pub player_token_account: Account<'info, TokenAccount>, 

    //Associated token account to store the enemy's treasure
    #[account(
        init,
        payer = owner,
        associated_token::mint = goldendragon_mint,
        associated_token::authority = game_data_account,
    )] 
    pub treasure_token_account: Account<'info, TokenAccount>,

    /// The Solana System Program, required to create new accounts on-chain.
    pub system_program: Program<'info, System>,
    ///Token programs
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>, 
}