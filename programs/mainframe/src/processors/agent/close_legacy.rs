use crate::errors::MainframeError;
use crate::instructions::agent::CloseLegacyAgent;
use anchor_lang::prelude::*;

/// Close legacy agent account (old format without agent_nft field)
/// This is a one-time cleanup instruction for agents created before Agent-NFT system
pub fn close_legacy_agent(ctx: Context<CloseLegacyAgent>) -> Result<()> {
    msg!("Closing legacy agent account...");
    msg!("  Account: {}", ctx.accounts.agent_account.key());

    // Read and validate account data in one scope
    let (account_owner, account_lamports) = {
        let agent_data = ctx.accounts.agent_account.data.borrow();

        msg!("  Size: {} bytes", agent_data.len());

        // Verify discriminator matches AgentAccount
        let expected_discriminator = [241, 119, 69, 140, 233, 9, 112, 50];
        if agent_data.len() < 8 {
            return err!(MainframeError::InvalidNFT);
        }

        for i in 0..8 {
            if agent_data[i] != expected_discriminator[i] {
                msg!("Invalid discriminator - not an AgentAccount");
                return err!(MainframeError::InvalidAccountRelationship);
            }
        }

        // Parse owner from old format (bytes 40-72)
        if agent_data.len() < 72 {
            msg!("Account too small - invalid format");
            return err!(MainframeError::InvalidNFT);
        }

        let owner_bytes = &agent_data[40..72];
        let owner = Pubkey::try_from(owner_bytes).map_err(|_| MainframeError::InvalidNFT)?;

        let lamports = ctx.accounts.agent_account.lamports();

        (owner, lamports)
    }; // Drop borrow here

    msg!("  Account owner: {}", account_owner);
    msg!("  Signer: {}", ctx.accounts.owner.key());

    // Verify signer is agent owner OR protocol authority (dual control)
    let is_owner = account_owner == ctx.accounts.owner.key();
    let is_authority = ctx.accounts.protocol_config.authority == ctx.accounts.owner.key();

    require!(is_owner || is_authority, MainframeError::Unauthorized);

    msg!(
        "  Signer role: {}",
        if is_owner {
            "Owner"
        } else {
            "Protocol Authority"
        }
    );
    msg!("✓ Legacy agent validated");

    // Transfer lamports to owner and close account
    ctx.accounts.agent_account.sub_lamports(account_lamports)?;
    ctx.accounts.owner.add_lamports(account_lamports)?;

    // Assign to system program (closes the account)
    ctx.accounts
        .agent_account
        .assign(&anchor_lang::system_program::ID);

    // Realloc to 0 (fully closes)
    ctx.accounts.agent_account.realloc(0, false)?;

    msg!("✓ Legacy agent closed");
    msg!("  Rent refunded: {} lamports", account_lamports);
    msg!("  Refunded to: {}", ctx.accounts.owner.key());

    Ok(())
}
