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
        let mut gamelist=&mut ctx.accounts.game_list;
        gamelist.game_type_index=1;
        gamelist.list=Vec::new();
        Ok(())
    }
    pub fn create_game_type(ctx:Context<CreateGameType>,entry_price:u64,max_game:u8,max_player:u8)->Result<()>{
        let gamelist = &mut ctx.accounts.game_list ;
        let auth=ctx.accounts.authority.key();

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
        let (game_pda, game_seed) = Pubkey::find_program_address(&[&[gamelist.game_type_index]], ctx.program_id );
        gamelist.game_type_index +=1;
        invoke(
            &system_instruction::transfer( &ctx.accounts.authority.key, &game_pda, 890880),
            &[
                ctx.accounts.authority.to_account_info(),
                ctx.accounts.game_pda.to_account_info(),
                ctx.accounts.system_program.to_account_info()
            ]
        )?;

        Ok(())
    }

    pub fn add_player(ctx: Context<AddPlayer>) -> Result<()>{

        let (treasury_pda, bump_seed) = Pubkey::find_program_address(&[b"Treasury"], ctx.program_id );
        //gametypepda 
        let solem_inc_pk = Pubkey::from_str("C8G8fK6G6tzPeFDXArqXPJusd1vDfQAftLwBNu3qmaRb").unwrap();
        let gamelist = &mut ctx.accounts.game_list;
        let entryprice =gamelist.list[0].entry_price;

        if ctx.accounts.player.lamports() >= entryprice {
            let game = &mut ctx.accounts.game ;
            if game.Player.len()==0 {
                game.rm=0;
            }
            let mut full = false ;
            let mut i = 0 ;
            let mut can_add = true ;
            loop {
                if i < game.Player.len() {
                    if game.Player[i].to_string() == ctx.accounts.player.key.to_string() {
                        can_add = false ;
                    } 
                }else {
                    break ;
                }
                i = i + 1 ;
            }
            if can_add{
                let mut i = 0 ;
           
                if i<gamelist.list[0].max_player{
                    game.Player.push(ctx.accounts.player.key());

                    invoke(
                        &system_instruction::transfer( &ctx.accounts.player.key, &treasury_pda, entryprice),
                        &[
                            ctx.accounts.player.to_account_info(),
                            ctx.accounts.treasury.to_account_info(),
                            ctx.accounts.system_program.to_account_info()
                        ]
                    )?;
                }
                if i >= gamelist.list[0].max_player{
                    full = true ;
                }else{
                    msg!("Player {} enter in game.", ctx.accounts.player.key.to_string()); 
                }
            if full{

                let treasury_funds = ctx.accounts.treasury.lamports() ;
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

                invoke_signed(
                    &system_instruction::transfer(&treasury_pda, &game_pda, final_reward),
                    &[
                        ctx.accounts.treasury.to_account_info(),
                        ctx.accounts.game_pda.to_account_info(),
                        ctx.accounts.system_program.to_account_info()
                    ],
                    &[&["Treasury".as_ref(),
                        &[bump_seed],
                    ]],
                )?;
                invoke_signed(
                    &system_instruction::transfer( &treasury_pda, &solem_inc_pk, comission),
                    &[
                        ctx.accounts.treasury.to_account_info(),
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

pub fn remove_player(ctx : Context<Remove>) -> Result<()> {
    let refund = ctx.accounts.game_list.list[0].entry_price;
        let (treasury_pda, bump_seed) = Pubkey::find_program_address(&[b"Treasury"], ctx.program_id );
        let game = &mut ctx.accounts.game ;
        let player = ctx.accounts.player.key.to_string() ;
        let mut i1 = 0 ;
        let mut playersInGame = game.Player.len() ;
        // if playersInGame < game.Player.ma     {

        // }

   
    Ok(())

}
#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    pub server: Signer<'info>,
    #[account(init,payer = server,space = 8 + 32*100)]
    pub game_list : Account<'info, GameList>,
    pub system_program : Program<'info, System>

}
#[derive(Accounts)]
pub struct CreateGameType<'info>{
    #[account(mut)]
    pub game_list : Account<'info, GameList>,
    
    #[account(mut)]
    /// CHECK:
    pub game_pda : AccountInfo<'info>,

    #[account(mut)]
    pub authority : Signer<'info>,

    #[account(init_if_needed,payer = authority, space = 9000,seeds = [b"GAME_TYPE".as_ref(),&[game_list.game_type_index]],bump)]
    pub gameType : Account<'info, GameType>,

    pub system_program : Program<'info, System>
}

#[derive(Accounts)]
pub struct AddPlayer<'info>{

    #[account(mut)]
    pub player:Signer<'info>,

    //#[account(mut)]
    //pub game_list : Account<'info, GameList>,

    /// CHECK:
    #[account(mut)]
    pub treasury : AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub solem_inc : AccountInfo<'info>,

    /// CHECK:
    #[account(mut)]
    pub game_pda : AccountInfo<'info>,

    #[account(mut)]
    pub authority : Signer<'info>,
    
    #[account(mut,seeds = [b"GAME_TYPE".as_ref(),&[game_list.game_type_index]],bump)]
    pub gameType : Account<'info, GameType>,
    
    #[account(init,payer = authority, space = 9000,seeds = [b"GAME".as_ref(),&[game_list.list[0].id]],bump)]
    // #[account(mut)]
    pub game : Account<'info, Game>,

    pub system_program : Program<'info, System>
}

#[derive(Accounts)]
pub struct Remove<'info>{


    #[account(mut)]
    pub game_list : Account<'info, GameList>,

    //CHECK : can be unsafe
    #[account(mut)]
    pub treasury : AccountInfo<'info>,

    pub system_program : Program<'info, System>,
  

    #[account(seeds = [b"GAME_TYPE".as_ref(),&[game_list.game_type_index]],bump)]
    pub gameType : Account<'info, GameType>,
   #[account(mut)]
    pub player : Signer<'info>,
  
    #[account(mut,seeds = [b"GAME".as_ref(),&[game_list.list[0].id]],bump)]
    // #[account(mut)]
    pub game : Account<'info, Game>,

}


#[account]
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
    pub running_index:u8,
}

#[account]
#[derive(Default)]
pub struct Game{
    pub game_type:Pubkey,
    pub Player:Vec<Pubkey>,
    pub winner:Pubkey,
    pub rm:u8,
    pub status:bool,
}

