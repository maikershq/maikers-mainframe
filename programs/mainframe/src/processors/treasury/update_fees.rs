use crate::instructions::UpdateFees;
use crate::state::FeeStructure;
use anchor_lang::prelude::*;

/// Update fee structure
pub fn update_fees(ctx: Context<UpdateFees>, new_fees: FeeStructure) -> Result<()> {
    ctx.accounts.protocol_config.fees = new_fees;
    Ok(())
}
