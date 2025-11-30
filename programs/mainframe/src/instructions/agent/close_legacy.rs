use crate::errors::MainframeError;
use crate::state::ProtocolConfig;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CloseLegacyAgent<'info> {
    /// Legacy agent account (old format without agent_nft field)
    #[account(mut)]
    /// CHECK: Old format agent - manually validated and closed in processor
    pub agent_account: UncheckedAccount<'info>,

    /// Owner (receives rent refund)
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        seeds = [b"protocol_config"],
        bump,
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,
}
