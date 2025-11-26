use crate::errors::MainframeError;
use crate::state::{AgentAccount, PartnerCollectionAccount, ProtocolConfig};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(nft_mint: Pubkey)]
pub struct CreateAgent<'info> {
    #[account(
        init,
        payer = owner,
        space = AgentAccount::LEN,
        seeds = [b"agent", nft_mint.as_ref()],
        bump
    )]
    pub agent_account: Account<'info, AgentAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,

    /// NFT token account (for SPL Token NFTs) or asset account (for Core NFTs)
    /// Optional to support multiple NFT standards:
    /// - SPL Token NFTs: provide token account (validated for ownership)
    /// - Core NFTs: provide asset account or None (validated via asset owner)
    /// - Compressed NFTs: requires merkle proof (future)
    #[account(mut)]
    /// CHECK: Manually validated in processor based on NFT type
    pub nft_token_account: Option<UncheckedAccount<'info>>,

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

    /// Optional affiliate wallet (receives commission)
    /// Can be zero-balance - will be funded by first commission
    #[account(mut)]
    /// CHECK: Any valid wallet can be an affiliate
    pub affiliate: Option<AccountInfo<'info>>,

    /// Optional affiliate account PDA (auto-initialized if needed)
    #[account(mut)]
    /// CHECK: PDA for affiliate stats, auto-initialized on first sale
    pub affiliate_account: Option<AccountInfo<'info>>,

    /// Optional referrer wallet (receives 5% if affiliate has referrer)
    #[account(mut)]
    /// CHECK: Any valid wallet can receive referrer commission
    pub referrer: Option<AccountInfo<'info>>,

    /// Optional referrer's affiliate account PDA (updated with referree stats)
    #[account(mut)]
    /// CHECK: PDA for referrer's stats, updated when referral makes sale
    pub referrer_account: Option<AccountInfo<'info>>,

    /// Optional partner collection account for discount validation
    /// PDA seeds validated in processor to prevent fake partner accounts
    pub partner_account: Option<Account<'info, PartnerCollectionAccount>>,

    /// Metaplex metadata account for collection verification
    /// CHECK: Validated in processor when collection_mint provided
    pub nft_metadata: Option<AccountInfo<'info>>,

    pub system_program: Program<'info, System>,
}
