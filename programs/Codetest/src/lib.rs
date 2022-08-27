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
        msg!(" L======>52");
        //let (global_treasury_pda, bump_seed) = Pubkey::find_program_address(&[b"Treasury"], ctx.program_id );
        let solem_inc_pk =
            Pubkey::from_str("C8G8fK6G6tzPeFDXArqXPJusd1vDfQAftLwBNu3qmaRb").unwrap();
        let gamelist = &mut ctx.accounts.game_list;
        let gametype = &mut ctx.accounts.game_type;
        //ctx.accounts.data.select_id=id;
        msg!(" L======>60");
        msg!(
            " l==========61 game key {} game last index {}",
            gametype.key(),
            gametype.last_game_index
        );
        // ctx.accounts.data.select_id_string=ctx.accounts.data.select_id_string.to_string();
        // let game_type=&mut ctx.accounts.game_type;
        let entryprice = gametype.entry_price;
        msg!(" L======>65");
        if ctx.accounts.player.lamports() >= entryprice {
            msg!(" L======>67");
            let game = &mut ctx.accounts.game_pda;
            if game.Players.len() == 0 {
                msg!(" L======>63");
                game.rm = 0;
            }
            msg!("gamme rewar multiplicator l====73 rm==={}", game.rm);
            let mut full = false;
            msg!("fullll initialize --------75----------> {}", full);
            let mut i: usize = 0;
            let mut can_add = true;

            // duplicate entry player check
            loop {
                msg!(" L======>72");
                if i < game.Players.len() {
                    msg!("<----i----> l====82==={}", i);
                    msg!(
                        "<----game.Players.len()----> l====83==={}",
                        game.Players.len()
                    );
                    msg!(" L======>74");

                    if game.Players[i].to_string() == ctx.accounts.player.key.to_string() {
                        msg!(" L======>76");
                        msg!("<----can_add----> l===88==={}", can_add);

                        can_add = false;
                        msg!("<----can_add----> l====102==={}", can_add);

                        //recheck iteration removal
                    }
                } else {
                    msg!(" L======>82");
                    break;
                }
                i = i + 1;
            }
            // now only player that are not duplicated
            if can_add {
                msg!(" L======>88");
                msg!(
                    "<----game.Players.len()----> l====103==={}",
                    game.Players.len()
                );
                let mut i = game.Players.len();
                msg!(
                    "<----game.Players.len()----> l====104==={}",
                    game.Players.len()
                );
                //added player public key in game.players
                // transfered player entry fee to game account
                msg!(
                    "<----gametype.max_player----> l====107==={}",
                    gametype.max_player
                );

                if i < gametype.max_player as usize {
                    msg!(" L======>132");
                    msg!(
                        "<----game.Players.len()----> l====111==={}",
                        game.Players.len()
                    );

                    game.Players.push(ctx.accounts.player.key());
                    i = game.Players.len(); // increased value of
                    
                    msg!("L141 =======> value of i ==== {}",i);
                    
                    msg!(
                        "<----game.Players.len()----> l====114==={}",
                        game.Players.len()
                    );

                    invoke(
                        &system_instruction::transfer(
                            &ctx.accounts.player.key,
                            &game.key(),
                            entryprice,
                        ),
                        &[
                            ctx.accounts.player.to_account_info(),
                            game.to_account_info(),
                            ctx.accounts.system_program.to_account_info(),
                        ],
                    )?;
                }
                msg!("L157====value of i -----------> {}", i);
                msg!("L158====value of full -----------> {}", full);
                msg!(
                    "L1590====value of gametype.max_player -----------> {}",
                    gametype.max_player as usize
                );
                msg!(
                    "L165====value of  game.Players.len() -----------> {}",
                    game.Players.len()
                );

                if i >= gametype.max_player as usize {
                    msg!(" L======>106");

                    full = true;

                    msg!("Updating value of full =======true====131===={}", full);
                } else {
                    msg!(" L======>109");
                    msg!(
                        "Player {} enter in game.",
                        ctx.accounts.player.key.to_string()
                    );
                }
                msg!(" L======>112");
                msg!(
                    "l=============123 game key {} game last index {}",
                    gametype.key(),
                    gametype.last_game_index
                );

                msg!("L179 ===========> value of full before exiting: {}", full);

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

    #[account(init_if_needed,payer = authority, space = 9000,seeds = [b"GAME".as_ref(),game_type.last_game_index_to_string.as_ref()],bump)]
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
