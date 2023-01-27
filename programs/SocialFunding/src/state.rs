use anchor_lang::prelude::*;

#[account]
pub struct Management {
    pub admin: Pubkey,
    pub pause: bool,
    pub executed: bool,
    pub project_stage: i64,
    pub voting_stage: i64,
    pub execute_stage: i64,
    pub donate_stage: i64,
    pub distribute_stage: i64,
}

#[derive(Accounts)]
pub struct Stage<'info> {
    #[account(init,payer=admin,space=32+1+8+8+8)]
    pub management: Account<'info, Management>,
    #[account(init,seeds=[b"sol_bank"],bump,payer=admin,space=8+8)]
    pub sol_bank: Account<'info, SolBank>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Pause<'info> {
    #[account(mut)]
    pub management: Account<'info, Management>,
    #[account(mut)]
    pub admin: Signer<'info>,
}

#[account]
pub struct SolBank {
    pub amount: u64,
    pub bump: u8,
    pub projects: Vec<Pubkey>,
    pub previous_project: Vec<Pubkey>,
    pub sol_counter: u64,
    pub project_amounts: Vec<u64>,
    pub project_counts: Vec<u64>,
}

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
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init,seeds=[b"member_counter",community.key().as_ref()],bump,payer=user,space=8+8)]
    pub member_counter: Account<'info, MemberCounter>,
    pub system_program: Program<'info, System>,
}

//permission olan bir community ise member eklemek için bir oylama yapılır

#[derive(Accounts)]
pub struct AddMembertoCommunity<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub community: Account<'info, Community>,
    #[account(
        mut,
        seeds = [b"member_counter", community.key().as_ref()],
        bump,
    )]
    pub member_counter: Account<'info, MemberCounter>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MemberCounter {
    pub counter: i64,
}

//-----------------------
//PROJECT
//-----------------------

#[account]
pub struct Project {
    pub creator: Pubkey,
    pub community: Pubkey,
    pub subject: String,
    pub description: String,
    pub executable: bool,
}

#[derive(Accounts)]
pub struct CreateProject<'info> {
    #[account(init,payer=creator,space=8+32+32+8+4*50+4*200)]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub management: Account<'info, Management>,
    pub community: Account<'info, Community>,
    #[account(init,seeds=[b"counter",project.key().as_ref()],bump,payer=creator,space=8+8+8)]
    pub counter: Account<'info, VoteCounter>,
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
    pub management: Account<'info, Management>,
    pub project: Account<'info, Project>,
    pub community: Account<'info, Community>,
    #[account(
        mut,
        seeds = [b"counter", project.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info, VoteCounter>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum VotingResult {
    Yes,
    No,
    Invalid,
}

impl VotingResult {
    pub fn validate(voting_char: char) -> Self {
        match voting_char {
            'Y' => Self::Yes,
            'N' => Self::No,
            _ => Self::Invalid,
        }
    }
}

#[account]
pub struct VoteCounter {
    pub yes_count: i64,
    pub no_count: i64,
}

#[derive(Accounts)]
pub struct ExecuteProject<'info> {
    #[account(mut)]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub management: Account<'info, Management>,
    pub community: Account<'info, Community>,
    #[account(
        mut,
        seeds = [b"counter", project.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info, VoteCounter>,
    #[account(mut,seeds=[b"sol_bank"],bump)]
    pub sol_bank: Account<'info, SolBank>,
}

//-----------------------
//DONATE TO THE PROJECT
//-----------------------

#[account]
pub struct Donate {
    pub amount: u64,
    pub donate_count: u64,
    pub timestamp: i64,
    pub donate_bump: u8,
}

#[derive(Accounts)]
#[instruction(project:Pubkey) ]

pub struct DonateProject<'info> {
    #[account(init,seeds=[b"donate",project.key().as_ref()],bump,payer=user,space=8+8+8+8)]
    pub donate: Account<'info, Donate>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub project: Account<'info, Project>,
    pub management: Account<'info, Management>,
    #[account(mut,seeds=[b"sol_bank"],bump)]
    pub sol_bank: Account<'info, SolBank>,
}

#[derive(Accounts)]
pub struct DistributeFunds<'info> {
    #[account(mut,seeds=[b"sol_bank"],bump)]
    pub sol_bank: Account<'info, SolBank>,
    pub management: Account<'info, Management>,
    pub user: Signer<'info>,
    pub project: Account<'info, Project>,
    #[account(seeds=[b"donate",project.key().as_ref()],bump)]
    pub donate: Account<'info, Donate>,
    pub system_program: Program<'info, System>,
}

//parayı çekebilmesi için community içinde oylama yapılır

#[account]
pub struct Withdraw {
    pub user: Pubkey,
    pub bump: u8,
    pub result: VotingResult,
    pub amount: u64,
    pub executed: bool,
    pub executable: bool,
}

#[derive(Accounts)]
pub struct AskForWithdraw<'info> {
    #[account(init,seeds=[b"withdraw",user.key().as_ref(),project.key().as_ref()],bump,payer=user,space=8+8+8+8)]
    pub withdraw: Account<'info, Withdraw>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub project: Account<'info, Project>,
    #[account(seeds=[b"donate",project.key().as_ref()],bump)]
    pub donate: Account<'info, Donate>,
    #[account(init,seeds=[b"withdraw_counter",project.key().as_ref(),user.key().as_ref()],bump,payer=user,space=8+8+8)]
    pub counter: Account<'info, VoteCounter>,
    pub community: Account<'info, Community>,
}

#[derive(Accounts)]
pub struct VotingWithdraw<'info> {
    #[account(seeds=[b"withdraw",user.key().as_ref(),project.key().as_ref()],bump)]
    pub withdraw: Account<'info, Withdraw>,
    #[account(mut)]
    pub community: Account<'info, Community>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub project: Account<'info, Project>,
    #[account(
        mut,
        seeds = [b"withdraw_counter", withdraw.user.as_ref(),project.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info, VoteCounter>,
    #[account(init,payer=user,space=8+32+32+8+2+8,seeds=[b"withdraw_voting",withdraw.user.as_ref(),project.key().as_ref(),user.key().as_ref()],bump)]
    pub voting: Account<'info, Voting>,
    pub system_program: Program<'info, System>,
}
