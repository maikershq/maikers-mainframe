use anchor_lang::prelude::*;
use anchor_spl::token_interface::TokenAccount;
use crate::errors::MainframeError;
use crate::state::{AgentAccount, ProtocolConfig, PartnerCollectionAccount};

#[derive(Accounts)]
#[instruction(nft_mint: Pubkey)]
pub struct CreateAgent<'info> {
    #[account(
        init,
        payer = protocol_authority,
        space = AgentAccount::LEN,
        seeds = [b"agent", nft_mint.as_ref()],
        bump
    )]
    pub agent_account: Account<'info, AgentAccount>,
    
    #[account(mut)]
    pub owner: Signer<'info>,

    /// Protocol authority that pays for account creation (for rent recovery)
    #[account(
        mut,
        constraint = protocol_authority.key() == protocol_config.authority @ MainframeError::Unauthorized
    )]
    pub protocol_authority: Signer<'info>,
    
    /// NFT token account owned by the user
    #[account(
        constraint = nft_token_account.mint == nft_mint,
        constraint = nft_token_account.owner == owner.key(),
        constraint = nft_token_account.amount == 1
    )]
    pub nft_token_account: InterfaceAccount<'info, TokenAccount>,
    
    /// NFT metadata account
    /// CHECK: Validated by program logic - existence and ownership verified
    pub nft_metadata: AccountInfo<'info>,
    
    /// Protocol configuration
    #[account(
        mut,
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
    
    /// Optional seller account that receives affiliate fee
    /// Can have zero balance - Solana allows transfers to any account passed in transaction
    #[account(mut)]
    /// CHECK: Must be a valid system account if provided. Supports zero-balance accounts.
    pub seller: Option<AccountInfo<'info>>,
    
    /// Optional partner collection account for discount validation
    /// CHECK: PDA validated in processor if collection_mint provided
    pub partner_account: Option<Account<'info, PartnerCollectionAccount>>,
    
    pub system_program: Program<'info, System>,
}
