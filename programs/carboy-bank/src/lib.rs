use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod carboy_bank {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("🦞 CarBoy-Bank Protocol Initialized");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
