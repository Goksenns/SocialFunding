use anchor_lang::prelude::*;
use errors::ErrorCode;
use state::*;

pub mod errors;
pub mod state;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod social_funding {

    use super::*;

    pub fn pause(ctx: Context<Stage>, pause: bool) -> Result<()> {
        let stage = &mut ctx.accounts.management;
        let admin = &mut ctx.accounts.admin;

        // ENV DOSYASI LAZIM
        stage.admin = Pubkey::new(&[
            160, 31, 10, 192, 18, 38, 7, 220, 161, 243, 36, 69, 11, 145, 13, 137, 102, 251, 202,
            220, 69, 242, 71, 65, 108, 125, 122, 185, 85, 221, 19, 135, 192, 11, 63, 131, 77, 26,
            55, 85, 10, 125, 55, 34, 190, 57, 104, 199, 188, 197, 96, 143, 51, 176, 41, 74, 102,
            232, 146, 107, 60, 138, 216, 189,
        ]);

        require!(stage.admin == admin.key(), ErrorCode::AuthenticationError);

        stage.pause = pause;

        if !pause {
            let clock = Clock::get().unwrap();

            stage.project_stage = clock.unix_timestamp + 60 * 60 * 24 * 3;
            stage.voting_stage = stage.project_stage + 60 * 60 * 24 * 5;
            stage.funding_stage = stage.voting_stage + 60 * 60 * 24 * 10;
        }

        Ok(())
    }

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
    ) -> Result<()> {
        let project = &mut ctx.accounts.project;
        let management = &mut ctx.accounts.management;
        let creator = &ctx.accounts.creator;
        let community = &mut ctx.accounts.community;
        let clock = Clock::get().unwrap();
        let counter = &mut ctx.accounts.counter;

        require!(!management.pause, ErrorCode::ContractPause);
        require!(
            management.project_stage < clock.unix_timestamp
                && management.voting_stage > clock.unix_timestamp,
            ErrorCode::NotInProjectStage
        );

        let mut is_this_member = false;
        for member in community.members.iter() {
            if &creator.key() == member {
                is_this_member = true;
                break;
            }
        }

        require!(is_this_member, ErrorCode::AuthenticationError);

        counter.no_count = 0;
        counter.yes_count = 0;

        project.subject = subject;
        project.description = description;
        project.creator = *creator.key;
        project.community = community.key();

        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, vote: String, voting_bump: u8) -> Result<()> {
        let voting = &mut ctx.accounts.voting;
        let project = &mut ctx.accounts.project;
        let counter = &mut ctx.accounts.counter;
        let clock = Clock::get().unwrap();

        let voting_char = VotingResult::validate(vote.chars().nth(0).unwrap());
        require!(voting_char != VotingResult::Invalid, ErrorCode::InvalidChar);

        if voting_char == VotingResult::Yes {
            counter.yes_count += 1;
        } else {
            counter.no_count += 1;
        }

        voting.user = *ctx.accounts.user.key;
        voting.project = project.key();
        voting.timestamp = clock.unix_timestamp;
        voting.result = voting_char;
        voting.bump = voting_bump;

        Ok(())
    }
}
