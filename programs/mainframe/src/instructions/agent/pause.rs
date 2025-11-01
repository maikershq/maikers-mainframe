use anchor_lang::prelude::*;
use anchor_spl::token_interface::TokenAccount;
use crate::errors::MainframeError;
use crate::state::{AgentAccount, AgentStatus, ProtocolConfig};

#[derive(Accounts)]
pub struct PauseAgent<'info> {
    #[account(
        mut,
        seeds = [b"agent", agent_account.nft_mint.as_ref()],
        bump,
        constraint = agent_account.owner == owner.key() @ MainframeError::Unauthorized,
        constraint = agent_account.status == AgentStatus::Active || agent_account.status == AgentStatus::Paused @ MainframeError::AgentNotActive
    )]
    pub agent_account: Account<'info, AgentAccount>,
    
    /// NFT token account - validates current owner actually owns the NFT
    #[account(
        constraint = nft_token_account.mint == agent_account.nft_mint @ MainframeError::InvalidNFT,
        constraint = nft_token_account.owner == owner.key() @ MainframeError::NFTNotOwned,
        constraint = nft_token_account.amount == 1 @ MainframeError::NFTNotOwned
    )]
    pub nft_token_account: InterfaceAccount<'info, TokenAccount>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        seeds = [b"protocol_config"],
        bump,
        constraint = !protocol_config.paused @ MainframeError::ProtocolPaused
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,
}
