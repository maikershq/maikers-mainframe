use crate::state::ProtocolConfig;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateTreasuryAddresses<'info> {
    #[account(
        mut,
        seeds = [b"protocol_config"],
        bump,
        has_one = authority @ crate::errors::MainframeError::Unauthorized
    )]
    pub protocol_config: Account<'info, ProtocolConfig>,

    /// Protocol authority (must match config)
    pub authority: Signer<'info>,

    /// CHECK: New protocol treasury (validated as valid pubkey)
    pub new_protocol_treasury: UncheckedAccount<'info>,

    /// CHECK: New validator treasury (validated as valid pubkey)
    pub new_validator_treasury: UncheckedAccount<'info>,

    /// CHECK: New network treasury (validated as valid pubkey)
    pub new_network_treasury: UncheckedAccount<'info>,
}
