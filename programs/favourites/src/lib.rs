use anchor_lang::prelude::*;

declare_id!("FFDh27pDckMexa5Nh8TmkmCRQZSrUBCBvVKtH5EU1VDM");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favourites {
    use super::*;

    pub fn set_favourites(
        context: Context<SetFavourites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        msg!("Hello there, Hope you doing great!");
        let user_public_key = context.accounts.user.key();
        msg!("User {:?} has his Favourites Number as {:?}, Favourite Color as {:?} and Favourite Hobbies as {:?}", user_public_key, number, color, hobbies);

        context.accounts.favourites.set_inner(Favourites {
            number,
            color,
            hobbies,
        });

        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Favourites {
    pub number: u64,

    #[max_len(50)]
    pub color: String,

    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

#[derive(Accounts)]
pub struct SetFavourites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favourites::INIT_SPACE,
        seeds = [b"favourites", user.key().as_ref()],
        bump 
    )]
    pub favourites: Account<'info, Favourites>,

    pub system_program: Program<'info, System>,
}
