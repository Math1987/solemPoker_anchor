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

        //let (global_treasury_pda, bump_seed) = Pubkey::find_program_address(&[b"Treasury"], ctx.program_id );
        let solem_inc_pk =
            Pubkey::from_str("C8G8fK6G6tzPeFDXArqXPJusd1vDfQAftLwBNu3qmaRb").unwrap();

        let gamelist = &mut ctx.accounts.game_list; // data account
        let gametype = &mut ctx.accounts.game_type; // data account
        let game = &mut ctx.accounts.game_pda; // PDA account

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

        if game.Players.len() < gametype.max_player as usize {
            if ctx.accounts.player.lamports() >= entryprice {
                msg!(" L======>67 lamports greater then required");

                if game.Players.len() == 0 {
                    msg!("L======>63 setting reward multiplicator as 0 when there are no players in the vector");
                    game.rm = 0; // setting reward multiplicator as 0
                }

                // else we are printing rm of each game PDA
                msg!("game reward multiplicator l====73 rm==={}", game.rm);

                // checking full state / room state of the game
                let mut full = false;
                msg!(
                    "🚀 ~ file: lib.rs ~ line 96 ~ if ctx.accounts.player.lamports ~ full {}",
                    full
                );

                let mut i = 0; // indexer for duplicate checking phase
                let mut can_add = true; // true by default

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

                            can_add = false; // terminate the instruction
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

                        // transfered player entry fee to game account

                        invoke(
                            &system_instruction::transfer(
                                &ctx.accounts.player.key,
                                &game.key(), // local var .key()
                                entryprice,
                            ),
                            &[
                                ctx.accounts.player.to_account_info(),
                                game.to_account_info(),
                                ctx.accounts.system_program.to_account_info(),
                            ],
                        )?;
                        msg!("Line 177: entry fee is successfully transferred to gamePda Account")
                    }
                    // entry fee is successfully transferred to gamePda Account

                    msg!(
                        "🚀 ~ file: lib.rs ~ line 182 ~ if ctx.accounts.player.lamports ~ full {}",
                        full
                    ); // full status
                    msg!("🚀 ~ file: lib.rs ~ line 186 ~ if ctx.accounts.player.lamports ~ gametype.max_player {}", gametype.max_player); // max_players in game

                    post_add_state = game.Players.len(); // In case of 3 players added, this value will be == 3 // repeated for convenience
                    msg!("🚀 ~ file: lib.rs ~ line 185 ~ ifctx.accounts.player.lamports ~ post_add_state {}", post_add_state);
                    if post_add_state as usize == (gametype.max_player) as usize {
                        // >=   ===>  ==

                        full = true;
                        msg!("🚀 ~ file: lib.rs ~ line 188 ~ ifctx.accounts.player.lamports ~ UPDATED full - PHASE 2 {}", full);
                    } else {
                        msg!(
                            "Player {} has entered in game, and entryfee is also deducted.",
                            ctx.accounts.player.key.to_string()
                        );
                    }

                    msg!(
                        "l=============123 game key {} game last index {}",
                        gametype.key(),
                        gametype.last_game_index
                    );

                    msg!("L179 ===========> value of full before exiting: {}", full);

                    // phase 3: once all players have taken place we should mark current gamePda account as full
                    if full {
                        msg!(
                            "l==========127 game key {} game last index {}",
                            gametype.key(),
                            gametype.last_game_index
                        );

                        msg!("You are inside Full");

                        gametype.last_game_index += 1;
                        gametype.last_game_index_to_string = gametype.last_game_index.to_string();
                        let treasury_funds = ctx.accounts.game_treasury_pda.lamports();
                        let now_ts = Clock::get().unwrap().unix_timestamp;
                        let random = now_ts % 1000 + 1;
                        let players_funds = 3 * entryprice * 9 / 10;
                        let players_funds = 3 * entryprice * 9 / 10;
                        msg!(" L======>133");
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
                        msg!(" L======>145");
                        let final_reward = entryprice * (game.rm as u64);
                        let (game_pda, game_seed) = Pubkey::find_program_address(
                            &[
                                b"GAME".as_ref(),
                                gametype.last_game_index_to_string.as_ref(),
                            ],
                            ctx.program_id,
                        );

                        let comission = entryprice * 3 / 10;

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
                        msg!(" L======>162");
                        invoke_signed(
                            &system_instruction::transfer(
                                &game_pda.key(),
                                &solem_inc_pk,
                                comission,
                            ),
                            &[
                                ctx.accounts.game_pda.to_account_info(),
                                ctx.accounts.solem_inc.to_account_info(),
                                ctx.accounts.system_program.to_account_info(),
                            ],
                            &[&[
                                "GAME".as_ref(),
                                gametype.last_game_index_to_string.as_ref(),
                                &[game_seed],
                            ]],
                        )?;
                        msg!(" L======>174");
                    }
                }
            }
        }
        if ctx.accounts.player.lamports() >= entryprice {
            msg!(" L======>67 lamports greater then required");

            if game.Players.len() == 0 {
                msg!("L======>63 setting reward multiplicator as 0 when there are no players in the vector");
                game.rm = 0; // setting reward multiplicator as 0
            }

            // else we are printing rm of each game PDA
            msg!("game reward multiplicator l====73 rm==={}", game.rm);

            // checking full state / room state of the game
            let mut full = false;
            msg!(
                "🚀 ~ file: lib.rs ~ line 96 ~ if ctx.accounts.player.lamports ~ full {}",
                full
            );

            let mut i = 0; // indexer
            let mut can_add = true; // true by default

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

                        can_add = false; // terminate the instruction
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

                    // transfered player entry fee to game account

                    invoke(
                        &system_instruction::transfer(
                            &ctx.accounts.player.key,
                            &game.key(), // local var .key()
                            entryprice,
                        ),
                        &[
                            ctx.accounts.player.to_account_info(),
                            game.to_account_info(),
                            ctx.accounts.system_program.to_account_info(),
                        ],
                    )?;
                    msg!("Line 177: entry fee is successfully transferred to gamePda Account")
                }
                // entry fee is successfully transferred to gamePda Account

                msg!(
                    "🚀 ~ file: lib.rs ~ line 182 ~ if ctx.accounts.player.lamports ~ full {}",
                    full
                ); // full status
                msg!("🚀 ~ file: lib.rs ~ line 186 ~ if ctx.accounts.player.lamports ~ gametype.max_player {}", gametype.max_player); // max_players in game

                post_add_state = game.Players.len(); // In case of 3 players added, this value will be == 3 // repeated for convenience
                msg!("🚀 ~ file: lib.rs ~ line 185 ~ ifctx.accounts.player.lamports ~ post_add_state {}", post_add_state);
                if post_add_state as usize == (gametype.max_player) as usize {
                    // >=   ===>  ==

                    full = true;
                    msg!("🚀 ~ file: lib.rs ~ line 188 ~ ifctx.accounts.player.lamports ~ UPDATED full - PHASE 2 {}", full);
                } else {
                    msg!(
                        "Player {} has already entered in game, and entryfee is also deducted.",
                        ctx.accounts.player.key.to_string()
                    );
                }

                msg!(
                    "l=============123 game key {} game last index {}",
                    gametype.key(),
                    gametype.last_game_index
                );

                msg!("L179 ===========> value of full before exiting: {}", full);

                // phase 3: once all players have taken place we should mark current gamePda account as full
                if full {
                    msg!(
                        "l==========127 game key {} game last index {}",
                        gametype.key(),
                        gametype.last_game_index
                    );

                    msg!("You are inside Full");

                    gametype.last_game_index += 1;
                    gametype.last_game_index_to_string = gametype.last_game_index.to_string();
                    let treasury_funds = ctx.accounts.game_treasury_pda.lamports();
                    let now_ts = Clock::get().unwrap().unix_timestamp;
                    let random = now_ts % 1000 + 1;
                    let players_funds = 3 * entryprice * 9 / 10;
                    let players_funds = 3 * entryprice * 9 / 10;
                    msg!(" L======>133");
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
                    msg!(" L======>145");
                    let final_reward = entryprice * (game.rm as u64);
                    let (game_pda, game_seed) = Pubkey::find_program_address(
                        &[
                            b"GAME".as_ref(),
                            gametype.last_game_index_to_string.as_ref(),
                        ],
                        ctx.program_id,
                    );

                    let comission = entryprice * 3 / 10;

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
                    msg!(" L======>162");
                    invoke_signed(
                        &system_instruction::transfer(&game_pda.key(), &solem_inc_pk, comission),
                        &[
                            ctx.accounts.game_pda.to_account_info(),
                            ctx.accounts.solem_inc.to_account_info(),
                            ctx.accounts.system_program.to_account_info(),
                        ],
                        &[&[
                            "GAME".as_ref(),
                            gametype.last_game_index_to_string.as_ref(),
                            &[game_seed],
                        ]],
                    )?;
                    msg!(" L======>174");
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
    // #[account(mut)]
    // pub global_treasury_pda : AccountInfo<'info>,
    /// CHECK:

    #[account(mut)]
    pub solem_inc: AccountInfo<'info>,

    /// CHECK:
    #[account(mut)]
    pub game_treasury_pda: AccountInfo<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,
    // #[account(mut,seeds = [b"GAME_TYPE".as_ref(),&[data.select_id]],bump)]
    //#[account(mut,seeds = [b"GAME_TYPE".as_ref(),data.select_id_string.as_ref()],bump)]
    #[account(mut)]
    pub game_type: Account<'info, GameType>,

    #[account(init_if_needed,payer = authority, space = 10000,seeds = [b"GAME".as_ref(),game_type.last_game_index_to_string.as_ref()],bump)]
    pub game_pda: Account<'info, Game>,

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
