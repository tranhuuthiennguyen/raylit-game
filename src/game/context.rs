use raylib::{RaylibHandle, RaylibThread};

use crate::game::state::GameState;

pub struct Context<'a> {
    pub state: &'a mut GameState,
    pub rl: &'a mut RaylibHandle,
    pub thread: &'a RaylibThread
}