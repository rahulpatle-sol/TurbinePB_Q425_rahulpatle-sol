use anchor_lang::prelude::*;

declare_id!("AwaVma7smGPKkPmEVobw1KnQtaR1jSSn3GWcwd6hwSp2");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello Solana! i love to build with you");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}