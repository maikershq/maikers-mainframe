use anchor_lang::prelude::*;
use crate::events::*;
use crate::instructions::TransferAgent;
use crate::state::ProtocolConfig;

/// Transfer agent ownership - one-sided operation where new NFT owner claims control
pub fn transfer_agent(ctx: Context<TransferAgent>) -> Result<()> {
    // NFT ownership is validated via account constraints
    // New owner pays to claim agent control (previous owner signature not required)
    
    let fee_amount = ctx.accounts.protocol_config.calculate_base_fee("transfer_agent");
    
    if fee_amount > 0 {
        ProtocolConfig::distribute_fee(&ctx.accounts.protocol_config,
            fee_amount,
            &ctx.accounts.new_owner.to_account_info(),
            &ctx.accounts.protocol_treasury.to_account_info(),
            &ctx.accounts.validator_treasury.to_account_info(),
            &ctx.accounts.network_treasury.to_account_info(),
            &ctx.accounts.system_program.to_account_info(),
        )?;
    }
    
    let old_owner = ctx.accounts.agent_account.owner;
    ctx.accounts.agent_account.owner = ctx.accounts.new_owner.key();
    ctx.accounts.agent_account.updated_at = Clock::get()?.unix_timestamp;
    
    emit!(AgentTransferred {
        agent_account: ctx.accounts.agent_account.key(),
        nft_mint: ctx.accounts.agent_account.nft_mint,
        old_owner,
        new_owner: ctx.accounts.new_owner.key(),
        timestamp: Clock::get()?.unix_timestamp,
    });
    
    Ok(())
}

