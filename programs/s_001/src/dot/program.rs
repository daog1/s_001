#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{id, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

#[account]
#[derive(Debug)]
pub struct Hello {
    pub bump: u8,
}

impl<'info, 'entrypoint> Hello {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedHello<'info, 'entrypoint>> {
        let bump = account.bump;

        Mutable::new(LoadedHello {
            __account__: account,
            __programs__: programs_map,
            bump,
        })
    }

    pub fn store(loaded: Mutable<LoadedHello>) {
        let mut loaded = loaded.borrow_mut();
        let bump = loaded.bump;

        loaded.__account__.bump = bump;
    }
}

#[derive(Debug)]
pub struct LoadedHello<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, Hello>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub bump: u8,
}

pub fn init_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut hello: Empty<Mutable<LoadedHello<'info, '_>>>,
) -> () {
    let mut bump = hello.bump.unwrap();
    let mut hello = hello.account.clone();

    assign!(hello.borrow_mut().bump, bump);
}
