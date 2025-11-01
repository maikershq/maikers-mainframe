use anchor_lang::prelude::*;
use crate::instructions::UpdateFees;
use crate::state::FeeStructure;

/// Update fee structure
pub fn update_fees(ctx: Context<UpdateFees>, new_fees: FeeStructure) -> Result<()> {
    ctx.accounts.protocol_config.fees = new_fees;
    Ok(())
}

