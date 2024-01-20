use anchor_lang::prelude::*;

declare_id!("EPJ35rHAvSfuyLCMt3S6x6J94NsqczkwbCeHn5JwwbMw");

#[program]
pub mod note_taker {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
