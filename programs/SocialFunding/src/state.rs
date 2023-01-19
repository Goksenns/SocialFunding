use anchor_lang::prelude::*;

//-----------------------
//COMMUNİTY
//-----------------------

#[account]
pub struct Community {
    pub members: Vec<Pubkey>,
    pub members_pool: Vec<Pubkey>,
    pub name: String,
    pub description: String,
    pub timestamp: i64,
    pub permission: bool,
}

#[derive(Accounts)]
pub struct CreateCommunity<'info> {
    #[account(init,payer=user,space=8+32+8)]
    pub community: Account<'info, Community>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct JoinCommunity<'info> {
    #[account(mut)]
    pub community: Account<'info, Community>,
    pub user: Signer<'info>,
}

//-----------------------
//PROJECT
//-----------------------

#[account]
pub struct Project {
    pub community: Pubkey,
    pub creator: Pubkey,
    pub timestamp: i64,
    pub subject: String,
    pub description: String,
    pub deadline: i64,
}
#[derive(Accounts)]
pub struct CreateProject<'info> {
    #[account(init,payer=creator,space=8+32+32+8+4*50+4*200)]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

//-----------------------
//VOTİNG PROJECT
//-----------------------
#[account]
pub struct Voting {
    pub user: Pubkey,
    pub project: Pubkey,
    pub timestamp: i64,
    pub result: VotingResult,
    pub bump: u8,
}
#[derive(Accounts)]
#[instruction(project:Pubkey)]
pub struct Vote<'info> {
    #[account(init,payer=user,space=8+32+32+8+2+8,seeds=[b"voting",user.key().as_ref(),project.key().as_ref()],bump)]
    pub voting: Account<'info, Voting>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum VotingResult {
    Yes,
    No,
}
