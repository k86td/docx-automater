use std::time::Duration;

use messages::{Id, Msg};
use tuirealm::{
    terminal::TerminalBridge, Application, AttrValue, Attribute,
    EventListenerCfg, NoUserEvent, PollStrategy, Update, adapter::crossterm,
};

use crate::{letter_counter::LetterCounter, model::Model, label::Label};

mod counter;
mod letter_counter;
mod messages;
mod model;
mod label;

fn main() {
    let mut model = Model::default();

    let _ = model.terminal.enter_alternate_screen();
    let _ = model.terminal.enable_raw_mode();

    while !model.quit {
        match model.app.tick(PollStrategy::Once) {
            Ok(msg) if msg.len() > 0 => {
                model.redraw = true;
                for m in msg.into_iter() {
                    let mut m = Some(m);
                    while m.is_some() {
                        m = model.update(m);
                    }
                }
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
            _ => {}
        }
        if model.redraw {
            model.view();
            model.redraw = false;
        }
    }

    // Terminate terminal
    let _ = model.terminal.leave_alternate_screen();
    let _ = model.terminal.disable_raw_mode();
    let _ = model.terminal.clear_screen();
}


