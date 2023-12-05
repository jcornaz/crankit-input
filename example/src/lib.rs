#![no_std]

use crankit_game_loop::{game_loop, Game, Playdate};
use crankit_input::ButtonsStateSource;
use playdate_sys::println;

pub struct MyGame;

impl Game for MyGame {
    fn new(_: &Playdate) -> Self {
        Self
    }

    fn update(&mut self, playdate: &Playdate) {
        let buttons = playdate.system.buttons_state();
        println!("{buttons:?}");
    }
}

game_loop!(MyGame);
