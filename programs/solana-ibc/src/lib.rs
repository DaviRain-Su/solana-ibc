use anchor_lang::prelude::*;

declare_id!("CaLXaXF4c1tnjQUY4VHxJFTqbUcszExy7oAsRKXxjZeS");

#[program]
pub mod solana_ibc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
