use tuirealm::{PollStrategy, Update};

use crate::testing::model::Model;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;

mod testing;

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
