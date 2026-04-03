pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;
pub mod events;

use anchor_lang::prelude::*;

pub use constants::*;
pub use error::*;
pub use instructions::*;
pub use state::*;
pub use utils::*;
pub use events::*;

declare_id!("GwL5nCWRMFF13vMbYF6VrgFxu4ExJxkY3gJ8gm7qjCjW");

#[program]
pub mod token_battle {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler_initialize(ctx)
    }

    pub fn attack(ctx: Context<Attack>) -> Result<()> {
        attack::handler_attack(ctx)
    }

    pub fn respawn(ctx: Context<Respawn>) -> Result<()> {
        respawn::handler_respawn(ctx)
    }
}
