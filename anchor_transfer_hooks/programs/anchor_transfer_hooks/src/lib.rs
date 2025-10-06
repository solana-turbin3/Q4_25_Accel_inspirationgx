pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("Afp7jFoh3TJiEPWqJnt3tTxFq6gZrrVwYGnh2PpTkRM4");

#[program]
pub mod anchor_transfer_hooks {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
