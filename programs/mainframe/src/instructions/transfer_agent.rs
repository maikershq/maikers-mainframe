use anchor_lang::prelude::*;
use anchor_spl::token_interface::TokenAccount;
use crate::errors::MainframeError;
use crate::state::{AgentAccount, AgentStatus, ProtocolConfig};

#[derive(Accounts)]
pub struct TransferAgent<'info> {
    #[account(
        mut,
        seeds = [b"agent", agent_account.nft_mint.as_ref()],
        bump,
        constraint = agent_account.status == AgentStatus::Active @ MainframeError::AgentNotActive,
        constraint = agent_account.owner == current_owner.key() @ MainframeError::Unauthorized
    )]
    pub agent_account: Account<'info, AgentAccount>,
    
    #[account(mut)]
    pub current_owner: Signer<'info>,
    
    /// New NFT token account (owned by new_owner)
    #[account(
        constraint = new_nft_token_account.mint == agent_account.nft_mint,
        constraint = new_nft_token_account.owner == new_owner.key(),
        constraint = new_nft_token_account.amount == 1
    )]
    pub new_nft_token_account: InterfaceAccount<'info, TokenAccount>,
    
    #[account(mut)]
    pub new_owner: Signer<'info>,
    
    #[account(
        seeds = [b"protocol_config"],
        bump,
        constraint = !protocol_config.paused @ MainframeError::ProtocolPaused
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,
    
    /// Fee distribution accounts
    #[account(
        mut,
        constraint = protocol_treasury.key() == protocol_config.protocol_treasury
    )]
    /// CHECK: Validated by protocol_config constraint
    pub protocol_treasury: AccountInfo<'info>,
    
    #[account(
        mut,
        constraint = validator_treasury.key() == protocol_config.validator_treasury
    )]
    /// CHECK: Validated by protocol_config constraint
    pub validator_treasury: AccountInfo<'info>,
    
    #[account(
        mut,
        constraint = network_treasury.key() == protocol_config.network_treasury
    )]
    /// CHECK: Validated by protocol_config constraint
    pub network_treasury: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}
