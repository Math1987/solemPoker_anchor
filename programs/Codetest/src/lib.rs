use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, program::invoke_signed, system_instruction},
};
use std::str::FromStr;

declare_id!("AxmxjMWgQcMQRCbT6oF4PMhJfNGY9YnWNZcm4TowE3H9");

#[program]
pub mod codetest {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        msg!(" L======>16");
        let mut gamelist = &mut ctx.accounts.game_list;
        gamelist.game_type_index = 1;
        //gamelist.game_type_index_to_string=gamelist.game_type_index.to_string();
        //ctx.accounts.data.select_id=0;
        //ctx.accounts.data.select_id_string=ctx.accounts.data.select_id.to_string();
        gamelist.list = Vec::new();
        Ok(())
    }
    
    pub fn create_game_type(
        ctx: Context<CreateGameType>,
        entry_price: u64,
        max_game: u8,
        max_player: u8,
    ) -> Result<()> {
        msg!(" L======>26");
        let gamelist = &mut ctx.accounts.game_list;
        let auth = ctx.accounts.authority.key();
        let gametype = &mut ctx.accounts.game_type;
        gametype.last_game_index = 1;
        gametype.last_game_index_to_string = gametype.last_game_index.to_string();
        gametype.authority = auth;
        gametype.entry_price = entry_price;
        gametype.max_player = max_player;
        gametype.id = gamelist.game_type_index;
        gametype.max_games = max_game;
        //let (game_type_pda, game_seed) = Pubkey::find_program_address(&[b"GAME_TYPE".as_ref(),gamelist.game_type_index.to_string().as_bytes()], ctx.program_id );
        let gamelisttype = GameListType {
            game_type_key: gametype.key(),
            id: gamelist.game_type_index,
            entry_price,
            max_game,
            max_player,
            authority: auth,
        };
        gamelist.list.push(gamelisttype);
        gamelist.game_type_index += 1;
        //gamelist.game_type_index_to_string=gamelist.game_type_index.to_string();
        Ok(())
    }

    pub fn add_player(ctx: Context<AddPlayer>) -> Result<()> {
        msg!("Line 56: we are inside add_player");

        let (global_treasury_pda, global_treasury_pda_bump_seed) =
            Pubkey::find_program_address(&[b"GlobalTreasury"], ctx.program_id); // this is used only for transferring commission to solemInc
        let solem_inc_pk =
            Pubkey::from_str("C8G8fK6G6tzPeFDXArqXPJusd1vDfQAftLwBNu3qmaRb").unwrap();

        let gamelist = &mut ctx.accounts.game_list; // data account
        let gametype = &mut ctx.accounts.game_type; // data account
        let game = &mut ctx.accounts.game_pda; // PDA account
        let globaltreasury = &mut ctx.accounts.global_treasury_pda; // PDA account

        //ctx.accounts.data.select_id=id;

        msg!(
            "🚀 ~ file: lib.rs ~ line 64 ~ pub fn add_player ~ gamelist account: {} ",
            gamelist.key()
        );
        msg!(
            " l==========61 gametype key {} gametype.last_game_index: {}",
            gametype.key(),
            gametype.last_game_index
        );
        msg!(
            "🚀 ~ file: lib.rs ~ line 64 ~ pub fn add_player ~ game PDA account: {} ",
            game.key()
        );

        // ctx.accounts.data.select_id_string=ctx.accounts.data.select_id_string.to_string();

        // let game_type=&mut ctx.accounts.game_type;

        let entryprice = gametype.entry_price; // local var

        // why < ?   and not ===>     <=
        // because the third player is entering using this instruction
        // that means the current game.Players.len() should be < gametype.max_player
        if game.Players.len() < gametype.max_player as usize {
            if ctx.accounts.player.lamports() >= entryprice {
                msg!(" L======>67 lamports greater then required");

                if game.Players.len() == 0 {
                    msg!("L======>63 setting reward multiplicator as 0 when there are no players in the vector");
                    game.rm = 0; // setting reward multiplicator as 0
                }

                // else we are printing rm of each game PDA
                msg!("game reward multiplicator l====73 rm==={}", game.rm);

                let mut i = 0; // indexer used for duplicate entry player check == Phase 1
                let mut can_add = true; // true by default == Phase 2
                                        // checking full state / room state of the game == Phase 3
                let mut full = false;
                msg!(
                    "🚀 ~ file: lib.rs ~ line 96 ~ if ctx.accounts.player.lamports ~ full {}",
                    full
                );

                // duplicate entry player check
                // will not work for first player
                // total 3 players:  => i=0;1;2; (total 3 times)
                loop {
                    msg!("L======>72 duplicate entry player check loop");
                    if i < game.Players.len() {
                        msg!(
                            "🚀 ~ file: lib.rs ~ line 115 ~ if i<game.Players.len ~ i {}",
                            i
                        );
                        msg!("🚀 ~ file: lib.rs ~ line 115 ~ if i<game.Players.len ~ game.Players.len() current players in room: {}", game.Players.len());

                        if game.Players[i].to_string() == ctx.accounts.player.key.to_string() {
                            msg!("Line 199: In this case the all players in room will be checked with current player passed, and match found here");
                            msg!("Line 120: Cannot add this player, terminating the instruction!");

                            can_add = false; // terminate the instruction in case of false
                            break;
                        }
                    } else {
                        msg!("Line 129: proceed for next phase i.e can_add");
                        break; // simple breaking when no match found; with can_add == true; proceed for next phase
                    }
                    i = i + 1;
                }

                // now only player that are not duplicated can enter this next phase
                if can_add {
                    msg!("Line 137: we are inside can_add phase, which is second phase");
                    msg!(
                        "<----game.Players.len()----> l====103==={}",
                        game.Players.len()
                    );
                    let mut pre_add_state: usize = 0; // by default we are saying that there are no players in gamePda
                    let mut post_add_state: usize = 0;

                    pre_add_state = game.Players.len(); //pre add
                    msg!("🚀 ~ file: lib.rs ~ line 146 ~ ifctx.accounts.player.lamports ~ pre_add_state {}", pre_add_state);

                    // added player public key in game.players
                    // transfered player entry fee to game account

                    msg!("🚀 ~ file: lib.rs ~ line 153 ~ if ctx.accounts.player.lamports ~ gametype.max_player {}", gametype.max_player);

                    if pre_add_state < gametype.max_player as usize {
                        // i<=gametype.max_player  => i<gametype.max_player
                        // because when pre_add_state 3==3, we'll be inside a new gamePda
                        msg!("Line 155: same gamePda (for 1,2,3 player), pre_add_state < gametype.max_player");

                        game.Players.push(ctx.accounts.player.key()); // 1st,2nd,3rd player
                        post_add_state = game.Players.len(); // In case of 3 players added, this value will be == 3
                        msg!("🚀 ~ file: lib.rs ~ line 159 ~ ifctx.accounts.player.lamports ~ post_add_state == game.Players.len() Your player is been successfully added: {}", post_add_state);
                        msg!("🚀 ~ file: lib.rs ~ line 160 ~ ifctx.accounts.player.lamports ~ pre_add_state Your player is been successfully added: {}", pre_add_state);

                        // transfered player entry fee to global_treasury_pda account
                        invoke(
                            &system_instruction::transfer(
                                &ctx.accounts.player.key,
                                // &game.key(), // local var .key()
                                &global_treasury_pda.key(), // local var .key()
                                entryprice,
                            ),
                            &[
                                ctx.accounts.player.to_account_info(),
                                // game.to_account_info(),
                                globaltreasury.to_account_info(),
                                ctx.accounts.system_program.to_account_info(),
                            ],
                        )?;
                        msg!("Line 177: entry fee is successfully transferred to global_treasury_pda Account")
                    }
                    // entry fee is successfully transferred to global_treasury_pda Account

                    // will still be false; not yet updated after initialized
                    msg!(
                        "🚀 ~ file: lib.rs ~ line 182 ~ if ctx.accounts.player.lamports ~ full {}",
                        full
                    ); // full status
                    msg!("🚀 ~ file: lib.rs ~ line 186 ~ if ctx.accounts.player.lamports ~ gametype.max_player {}", gametype.max_player); // max_players in game

                    post_add_state = game.Players.len(); // In case of 3 players added, this value will be == 3 // repeated for convenience
                    msg!("🚀 ~ file: lib.rs ~ line 185 ~ ifctx.accounts.player.lamports ~ post_add_state {}", post_add_state);
                    if post_add_state as usize == (gametype.max_player) as usize {
                        // >=   ===>  ==

                        full = true; // here the value of full gets updated
                        msg!("🚀 ~ file: lib.rs ~ line 188 ~ if ctx.accounts.player.lamports ~ UPDATED full - PHASE 2 {}", full);
                        msg!(
                            "Player {} has entered in game, and entryfee is also deducted. And also the game if full",
                            ctx.accounts.player.key.to_string()
                        );
                        msg!(
                            "Line 200: gametype key {} gametype last_game_index {}",
                            gametype.key(),
                            gametype.last_game_index
                        );
                        msg!(
                            "Line 205: game key {} game struct game.Players {:#?}",
                            game.key(),
                            game.Players
                        );
                    } else {
                        msg!(
                            "Player {} has entered in game, and entryfee is also deducted. But the game has still some space",
                            ctx.accounts.player.key.to_string()
                        );
                        msg!(
                            "Line 211: gametype key {} gametype last_game_index {}",
                            gametype.key(),
                            gametype.last_game_index
                        );
                    }

                    // // phase 3: once all players have fulfilled the room space, we should mark current gamePda account as full, and increase the counter of gameType account
                    // // so that from next time from client side they can direct to next gamePda
                    // // 2 main tasks occurs here: Updates last_game_index+=1 in game_type account
                    // // transfers commission only when game room is full
                    if full {
                        msg!("You are inside Full");
                        msg!(
                            "Line 224: Current gametype.key {} Current gametype.last_game_index {}",
                            gametype.key(),
                            gametype.last_game_index
                        );

                        // After entering third player: 3/3 == full
                        // we have to update last_game_index
                        // this will create a new gamePda from client side next time based on

                        gametype.last_game_index += 1; // here is the update that we are looking at.

                        gametype.last_game_index_to_string = gametype.last_game_index.to_string(); // string type

                        msg!("Line 248: Below logic is only for setting up game reward multiplicator");
                        // let treasury_funds = ctx.accounts.game_pda.to_account_info().lamports.borrow(); // donator_program_account.to_account_info().try_borrow_mut_lamports()?
                        let treasury_funds = globaltreasury.lamports(); // In case of game_treasury_pda // // let treasury_funds = ctx.accounts.game_treasury_pda.lamports();

                        let now_ts = Clock::get().unwrap().unix_timestamp;
                        let random = now_ts % 1000 + 1;
                        let players_funds = 3 * entryprice * 9 / 10;

                        // Logic Implementation in Rust Issue: binary operation `>=` cannot be applied to type `Ref<'_, &mut u64>`rustcE0369
                        if random > 690 + 210 + 70 + 29 && treasury_funds >= players_funds * 50 {
                            game.rm = 50;
                        } else if random > 690 + 210 + 70 && treasury_funds >= players_funds * 10 {
                            game.rm = 10;
                        } else if random > 690 + 210 && treasury_funds >= players_funds * 5 {
                            game.rm = 5;
                        } else if random > 690 && treasury_funds >= players_funds * 3 {
                            game.rm = 3;
                        } else {
                            game.rm = 2;
                        }

                        // // Hard coded rm value for the time being when using gamePda itself as a global_treasury_pda
                        // game.rm = 2;

                        msg!(" L======>145");
                        // let final_reward = entryprice * (game.rm as u64); // no more required to send to each game_treasury_account;
                        // already taken in global_treasury_pda account

                        let gametype_previous_last_game_index = gametype.last_game_index - 1;
                        let gametype_previous_last_game_index_string =
                            gametype_previous_last_game_index.to_string();

                        let (game_pda, game_seed) = Pubkey::find_program_address(
                            &[
                                b"GAME".as_ref(),
                                // gametype.last_game_index_to_string.as_ref(),
                                gametype_previous_last_game_index_string.as_ref(),
                            ],
                            ctx.program_id,
                        );
                        msg!("🚀 ~ file: lib.rs ~ line 286 ~ ifctx.accounts.player.lamports ~ game_seed {}", game_seed);
                        msg!("🚀 ~ file: lib.rs ~ line 287 ~ ifctx.accounts.player.lamports ~ game_pda {}", game_pda);
                        msg!("🚀 ~ file: lib.rs ~ line 288 ~ ifctx.accounts.player.lamports ~ gametype_previous_last_game_index_string {}", gametype_previous_last_game_index_string);

                        let comission = entryprice * 3 / 10;

                        // transfer final_reward from global_treasury_pda to game_pda // no more required, can be done using global_treasury_itself
                        // invoke_signed(
                        //     &system_instruction::transfer(&global_treasury_pda, &game.key(), final_reward),
                        //     &[
                        //         ctx.accounts.global_treasury_pda.to_account_info(),
                        //         ctx.accounts.game_pda.to_account_info(),
                        //         ctx.accounts.system_program.to_account_info()
                        //     ],
                        //     &[&["Treasury".as_ref(),
                        //         &[bump_seed],
                        //     ]],
                        // )?;
                        msg!("Line 293: About to transfer commission to SolemInc");

                        // // 'Program 11111111111111111111111111111111 invoke [2]', 'Transfer: `from` must not carry data', 'Program 11111111111111111111111111111111 failed: invalid program argument'
                        invoke_signed(
                            &system_instruction::transfer(
                                &global_treasury_pda,
                                &solem_inc_pk,
                                comission,
                            ),
                            &[
                                ctx.accounts.global_treasury_pda.to_account_info(), // &globaltreasury.key()
                                ctx.accounts.solem_inc.to_account_info(),
                                ctx.accounts.system_program.to_account_info(),
                            ],
                            &[&[
                                // In case of GamePDA
                                // "GAME".as_ref(),
                                // // gametype.last_game_index_to_string.as_ref(), // this is already updated; and hence we need to take the existing value
                                // gametype_previous_last_game_index_string.as_ref(), // this is already updated; and hence we need to take the existing value
                                // &[game_seed],

                                // In case of GlobalTreasuryPDA
                                "GlobalTreasury".as_ref(), // TREASURY_PDA_SEED.as_ref(),
                                &[global_treasury_pda_bump_seed],
                            ]],
                        )?;
                        msg!("Line 311, Commission transferred to solem inc only when game room is full");
                    }
                }
            }
        }
        Ok(())
    }
}

// pub fn remove_player(ctx : Context<Remove>) -> Result<()> {
//     let refund = ctx.accounts.game_list.list[0].entry_price;
//         let (treasury_pda, bump_seed) = Pubkey::find_program_address(&[b"Treasury"], ctx.program_id );
//         let game = &mut ctx.accounts.game ;
//         let player = ctx.accounts.player.key.to_string() ;
//         let mut i1 = 0 ;
//         let mut playersInGame = game.Player.len() ;
//         // if playersInGame < game.Player.ma     {

//         // }

//     Ok(())

// }
#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    pub server: Signer<'info>,
    //#[account(init,payer = server,space = 10000,seeds = [b"GAME_LIST".as_ref()],bump)]
    #[account(init, payer = server, space = 10000)]
    pub game_list: Account<'info, GameList>,

    // #[account(init, payer = server, space = 9000)]
    // pub data : Account<'info,Data>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CreateGameType<'info> {
    //#[account(mut,seeds = [b"GAME_LIST".as_ref()],bump)]
    #[account(mut)]
    pub game_list: Account<'info, GameList>,

    // #[account(mut)]
    // /// CHECK:
    // pub game_treasury_pda : AccountInfo<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,

    // #[account(init,payer = authority, space = 9000,seeds = [b"GAME_TYPE".as_ref(),&[game_list_pda.game_type_index]],bump)]
    // #[account(init,payer = authority, space = 9000,seeds = [b"GAME_TYPE".as_ref(),b"1".as_ref()],bump)] // hardcoded is working
    //#[account(init,payer = authority, space = 9000,seeds = [b"GAME_TYPE".as_ref(),game_list_pda.game_type_index_to_string.as_ref()],bump)] // hardcoded is working
    #[account(init,payer = authority, space = 9000)]
    pub game_type: Account<'info, GameType>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddPlayer<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    // #[account(mut)]
    // pub data:Account<'info,Data>,

    //#[account(mut,seeds = [b"GAME_LIST".as_ref()],bump)]
    #[account(mut)]
    pub game_list: Account<'info, GameList>,

    /// CHECK:
    #[account(mut)]
    pub global_treasury_pda: AccountInfo<'info>, // since we want to use .lamports() method

    /// CHECK:
    #[account(mut)]
    pub solem_inc: AccountInfo<'info>,

    // /// CHECK:
    // #[account(mut)]
    // pub game_treasury_pda: AccountInfo<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    // #[account(mut,seeds = [b"GAME_TYPE".as_ref(),&[data.select_id]],bump)]
    //#[account(mut,seeds = [b"GAME_TYPE".as_ref(),data.select_id_string.as_ref()],bump)]
    #[account(mut)]
    pub game_type: Account<'info, GameType>,

    // #[account(init,payer = authority, space = 10000,seeds = [b"GAME".as_ref(),game_type.last_game_index_to_string.as_ref()],bump)] // will break the code //  'Allocate: account Address { address: 6RM3NZ7BA1R1zw9ZvxyCyJvh3jgSmkLXJBfxN1XLJEfN, base: None } already in use'
    #[account(init_if_needed,payer = authority, space = 10000,seeds = [b"GAME".as_ref(),game_type.last_game_index_to_string.as_ref()],bump)]
    pub game_pda: Account<'info, Game>, // this isnt AccountInfo, in which we can direcly use .lamports()

    pub system_program: Program<'info, System>,
}

// #[derive(Accounts)]
// pub struct Remove<'info>{

//     #[account(mut)]
//     pub game_list : Account<'info, GameList>,

//     //CHECK : can be unsafe
//     #[account(mut)]
//     pub global_treasury_pda : AccountInfo<'info>,

//     pub system_program : Program<'info, System>,

//     #[account(seeds = [b"GAME_TYPE".as_ref(),&[game_list.game_type_index]],bump)]
//     pub gameType : Account<'info, GameType>,
//    #[account(mut)]
//     pub player : Signer<'info>,

//     #[account(mut,seeds = [b"GAME".as_ref(),&[id]],bump)]
//     // #[account(mut)]
//     pub game : Account<'info, Game>,

// }

// #[account]
// pub struct Data{
//     pub select_id:u8,
//     pub select_id_string :String,
// }

#[account]
#[derive(Default)]
pub struct GameList {
    pub list: Vec<GameListType>,
    pub game_type_index: u8,
    //pub game_type_index_to_string: String,
}
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct GameListType {
    pub game_type_key: Pubkey,
    pub authority: Pubkey,
    pub id: u8,
    pub entry_price: u64,
    pub max_game: u8,
    pub max_player: u8,
}

#[account]
#[derive(Default)]
pub struct GameType {
    pub id: u8,
    pub authority: Pubkey,
    pub entry_price: u64,
    pub max_player: u8,
    pub max_games: u8,
    pub last_game_index: u8,
    pub last_game_index_to_string: String,
}

#[account]
#[derive(Default)]
pub struct Game {
    pub game_type: Pubkey,
    pub Players: Vec<Pubkey>,
    pub winner: Pubkey,
    pub rm: u8,
    pub status: bool,
}
