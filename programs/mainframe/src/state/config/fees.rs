use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct FeeStructure {
    pub create_agent: u64,
    pub update_agent_config: u64,
    pub transfer_agent: u64,
    pub pause_agent: u64,
    pub close_agent: u64,
    pub execute_action: u64,
}
