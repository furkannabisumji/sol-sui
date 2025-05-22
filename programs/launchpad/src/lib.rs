use anchor_lang::prelude::*;

declare_id!("okq6VRRARrgVJNQFnESNMDG3d3GEyVwGzacNGm1MCw2");

#[program]
pub mod launchpad {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from my first program", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
