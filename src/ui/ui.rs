use crate::tic_toc::game_field::GameField;

pub enum Mode {
    GUI,
    CLI,
}

pub enum Event {
    Quit,
    Point((usize, usize)),
    None,
}

pub trait UI {
    fn display(&mut self, game_field: &GameField);
    fn process_input(&mut self) -> Event;
}