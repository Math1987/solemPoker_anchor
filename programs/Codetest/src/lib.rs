use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke_signed, program::invoke, system_instruction}
    };
use std::str::FromStr;


declare_id!("6r5XBwuuinUGyTBJJ5CB4k4o9isBzyqjG18LUEnUfFNi");


#[program]
pub mod codetest {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        let mut gamelist=&mut ctx.accounts.game_list_pda;
        gamelist.game_type_index=1;
        ctx.accounts.data.select_id=0;
        gamelist.list=Vec::new();
        Ok(())
    }
    pub fn create_game_type(ctx:Context<CreateGameType>,entry_price:u64,max_game:u8,max_player:u8)->Result<()>{
        let gamelist = &mut ctx.accounts.game_list ;
        let auth=ctx.accounts.authority.key();
        let gametype=&mut ctx.accounts.game_type_pda;
        gametype.last_game_index=1;
        gametype.authority=auth;
        gametype.entry_price=entry_price;
        gametype.max_player=max_player;
        gametype.id=gamelist.game_type_index;
        gametype.max_games=max_game;
        let (game_type_pda, game_seed) = Pubkey::find_program_address(&[b"GAME_TYPE".as_ref(),&[gamelist.game_type_index]], ctx.program_id );
        let gamelisttype= GameListType{
            game_type_pda,
            id: gamelist.game_type_index,
            entry_price,
            max_game,
            max_player,
            authority:auth,

        };
        gamelist.list.push(gamelisttype);
        gamelist.game_type_index +=1;
        Ok(())
    }

    pub fn add_player(ctx: Context<AddPlayer>,id:u8) -> Result<()>{
        let (global_treasury_pda, bump_seed) = Pubkey::find_program_address(&[b"Treasury"], ctx.program_id );
        let solem_inc_pk = Pubkey::from_str("C8G8fK6G6tzPeFDXArqXPJusd1vDfQAftLwBNu3qmaRb").unwrap();
        let gamelist = &mut ctx.accounts.game_list_pda;
        ctx.accounts.data.select_id=id; 
        let game_type=&mut ctx.accounts.game_type_pda;
        let entryprice =ctx.accounts.game_type_pda.entry_price;
        if ctx.accounts.player.lamports() >= entryprice {
            let game = &mut ctx.accounts.game_pda ;
            if game.Players.len()==0 {
                game.rm=0;
            }
            let mut full = false ;
            let mut i = 0 ;
            let mut can_add = true ;

            // duplicate entry player check
            loop {
                if i < game.Players.len() {
                    if game.Players[i].to_string() == ctx.accounts.player.key.to_string() {
                        can_add = false ;
                        break;     //recheck iteration removal
                    } 
                }else {
                    break ;
                }
                i = i + 1 ;
            }
            if can_add{
                let mut i = game.Players.len() as u8 ;
                if i<ctx.accounts.game_type_pda.max_player{
                    game.Players.push(ctx.accounts.player.key());
                    
                    invoke(
                        &system_instruction::transfer( &ctx.accounts.player.key, &game.key(), entryprice),
                        &[
                            ctx.accounts.player.to_account_info(),
                            game.to_account_info(),
                            ctx.accounts.system_program.to_account_info()
                        ]
                    )?;
                }else{
                    full=true;
                }
                if i >= ctx.accounts.game_type_pda.max_player{
                    full = true ;
                }else{
                    msg!("Player {} enter in game.", ctx.accounts.player.key.to_string()); 
                }
            if full{

                ctx.accounts.game_type_pda.last_game_index += 1;
                let treasury_funds = ctx.accounts.game_treasury_pda.lamports() ;
                let now_ts = Clock::get().unwrap().unix_timestamp ;
                let random = now_ts%1000 + 1  ;
                let players_funds = 3*entryprice*9/10 ;
                let players_funds = 3*entryprice*9/10 ;

                if random > 690 + 210 + 70 + 29 && treasury_funds >= players_funds*50 {
                    game.rm = 50 ;
                } else if random > 690 + 210 + 70 && treasury_funds >= players_funds*10 {
                    game.rm = 10 ;
                } else if random > 690 + 210 && treasury_funds >= players_funds*5   {
                    game.rm = 5 ;
                } else if random > 690 && treasury_funds >= players_funds*3  {
                    game.rm = 3 ;
                }else{
                    game.rm = 2 ;
                }

                let final_reward = entryprice*(game.rm as u64) ;
                let (game_pda, game_seed) = Pubkey::find_program_address(&[&game.key().to_bytes()], ctx.program_id );

                let comission = entryprice*3/10 ;

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
                invoke_signed(
                    &system_instruction::transfer( &game_pda.key(), &solem_inc_pk, comission),
                    &[
                        ctx.accounts.game_pda.to_account_info(),
                        ctx.accounts.solem_inc.to_account_info(),
                        ctx.accounts.system_program.to_account_info()
                    ],
                    &[&[
                        "Treasury".as_ref(),
                        &[bump_seed],
                    ]],
                )?;
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
    #[account(init,payer = server,space = 10000,seeds = [b"GAME_LIST".as_ref()],bump)]
    pub game_list_pda : Account<'info, GameList>,
    #[account(init, payer = server, space = 9000)]
    pub data : Account<'info,Data>,
    pub system_program : Program<'info, System>

}
#[derive(Accounts)]
pub struct CreateGameType<'info>{
    #[account(mut,seeds = [b"GAME_LIST".as_ref()],bump)]
    pub game_list : Account<'info, GameList>,
    
    // #[account(mut)]
    // /// CHECK:
    // pub game_treasury_pda : AccountInfo<'info>,

    #[account(mut)]
    pub authority : Signer<'info>,

    #[account(init_if_needed,payer = authority, space = 9000,seeds = [b"GAME_TYPE".as_ref(),&[game_list.game_type_index]],bump)]
    pub game_type_pda : Account<'info, GameType>,

    pub system_program : Program<'info, System>
}

#[derive(Accounts)]
pub struct AddPlayer<'info>{

    #[account(mut)]
    pub player:Signer<'info>,

    #[account(mut)]
    pub data:Account<'info,Data>,

    #[account(mut,seeds = [b"GAME_LIST".as_ref()],bump)]
    pub game_list_pda : Account<'info, GameList>,

    /// CHECK:
    #[account(mut)]
    pub global_treasury_pda : AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub solem_inc : AccountInfo<'info>,

    /// CHECK:
    #[account(mut)]
    pub game_treasury_pda : AccountInfo<'info>,

    #[account(mut)]
    pub authority : Signer<'info>,
    
    #[account(mut,seeds = [b"GAME_TYPE".as_ref(),&[data.select_id]],bump)]
    pub game_type_pda : Account<'info, GameType>,
    
    #[account(init_if_needed,payer = authority, space = 9000,seeds = [b"GAME".as_ref(),&[game_type_pda.last_game_index]],bump)]
    pub game_pda : Account<'info, Game>,

    pub system_program : Program<'info, System>
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

#[account]
pub struct Data{
    pub select_id:u8,
}

#[account]
#[derive(Default)]
pub struct GameList{
    pub list:Vec<GameListType>,
    pub game_type_index: u8,

}
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct GameListType {
    pub game_type_pda : Pubkey,
    pub authority:Pubkey,
    pub id: u8,
    pub entry_price: u64,
    pub max_game:u8,
    pub max_player:u8,
}

#[account]
#[derive(Default)]
pub struct GameType{
    pub id:u8,
    pub authority:Pubkey,
    pub entry_price:u64,
    pub max_player:u8,
    pub max_games:u8,
    pub last_game_index:u8,
}

#[account]
#[derive(Default)]
pub struct Game{
    pub game_type:Pubkey,
    pub Players:Vec<Pubkey>,
    pub winner:Pubkey,
    pub rm:u8,
    pub status:bool,
}

