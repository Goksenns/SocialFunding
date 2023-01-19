use anchor_lang::prelude::*;
use errors::ErrorCode;
use state::*;

pub mod errors;
pub mod state;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod social_funding {
    use super::*;

    pub fn create_community(
        ctx: Context<CreateCommunity>,
        name: String,
        description: String,
        members: Vec<Pubkey>,
        permission: bool,
    ) -> Result<()> {
        let community = &mut ctx.accounts.community;
        let user = &ctx.accounts.user;
        let clock = Clock::get().unwrap();

        require!(members.len() >= 4, ErrorCode::InsufficientNumber);

        for member in members {
            community.members.push(member);
        }

        // userı signer olarak alıyoruz onun için vektöre userın keyini atıyoruz
        // members parametresinden ise direkt pubkey alıyoruz
        community.members.push(user.key());
        community.timestamp = clock.unix_timestamp;
        community.name = name;
        community.description = description;
        community.permission = permission;

        Ok(())
    }
    pub fn join_community(ctx: Context<JoinCommunity>) -> Result<()> {
        let community = &mut ctx.accounts.community;
        let user = &ctx.accounts.user;

        if community.permission == false {
            community.members.push(user.key());
        } else {
            community.members_pool.push(user.key());
        }

        Ok(())
    }
    pub fn create_project(
        ctx: Context<CreateProject>,
        subject: String,
        description: String,
        community: Pubkey,
    ) -> Result<()> {
        Ok(())
    }
}
