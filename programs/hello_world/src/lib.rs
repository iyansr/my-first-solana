use anchor_lang::prelude::*;

declare_id!("BKySJ174KyKLjG73QSVq4Gny334qFArmnpUeAPMACrAh");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello Madafaka from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
