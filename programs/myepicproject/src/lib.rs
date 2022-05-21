use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("9GqLCWyNJMzTcGNw9jJMa4SVnJSg1Nn9c4XLgs3ayKNY");

#[program]
pub mod myepicproject {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs = 0;
        Ok(())
    }

    // The function now accepts a gif_link param from the user. We also reference the user from the Context
    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        // Build the struct.
        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
            // upvotes: 0,
            // downvotes: 0,
            // id: base_account.total_gifs + 1,
        };

        // Add it to the gif_list vector.
        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }

    // pub fn set_vote(ctx: Context<AddGif>, gif_id: u64, up_or_down_vote: bool, adding_or_subtracting: bool) -> ProgramResult {

    //     let base_account = &mut ctx.accounts.base_account;

    //     for n in base_account.gif_list.iter_mut() {

    //         if gif_id == n.id {

    //             if up_or_down_vote {
    //                 if adding_or_subtracting {
    //                     n.upvotes += 1;
    //                 }
    //                 else {
    //                     n.upvotes -= 1;
    //                 }
    //             }

    //             else {
    //                 if adding_or_subtracting {
    //                     n.downvotes += 1;
    //                 }
    //                 else {
    //                     n.downvotes -= 1;
    //                 }
    //             }
    //         }
    //     }

    //     Ok(())
    // }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 10000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Add the signer who calls the AddGif method to the struct so that we can save it
#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    // pub upvotes: u64,
    // pub downvotes: u64,
    // pub id: u64,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    // Attach a Vector of type ItemStruct to the account.
    pub gif_list: Vec<ItemStruct>,
}
