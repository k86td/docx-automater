use std::time::Duration;

use tuirealm::{
    terminal::TerminalBridge,
    tui::prelude::{Constraint, Direction, Layout},
    Application, AttrValue, Attribute, EventListenerCfg, NoUserEvent, Update,
};

use crate::{
    testing::label::Label,
    testing::letter_counter::LetterCounter,
    testing::messages::{Id, Msg},
};

pub struct Model {
    pub app: Application<Id, Msg, NoUserEvent>,
    pub quit: bool,
    pub redraw: bool,
    pub terminal: TerminalBridge,
}

impl Model {
    pub fn view(&mut self) {
        assert!(self
            .terminal
            .raw_mut()
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Length(3), Constraint::Length(3)].as_ref())
                    .split(f.size());

                self.app.view(&Id::LetterCounter, f, chunks[0]);
                // self.app.view(&Id::Label, f, chunks[1])
            })
            .is_ok());
    }

    pub fn init_app() -> Application<Id, Msg, NoUserEvent> {
        let mut app = Application::init(
            EventListenerCfg::default()
                .default_input_listener(Duration::from_millis(20))
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_secs(1)),
        );

        assert!(app
            .mount(
                Id::LetterCounter,
                Box::new(LetterCounter::new(0)),
                Vec::default()
            )
            .is_ok());

        assert!(app
            .mount(Id::Label, Box::new(Label::default()), Vec::default())
            .is_ok());

        assert!(app.active(&Id::LetterCounter).is_ok());

        app
    }
}

impl Update<Msg> for Model {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        if let Some(msg) = msg {
            self.redraw = true;

            match msg {
                Msg::AppClose => {
                    self.quit = true;
                    None
                }
                Msg::DigitCounterChanged(_) => todo!(),
                Msg::DigitCounterBlur => todo!(),
                Msg::LetterCounterChanged(v) => {
                    assert!(self
                        .app
                        .attr(
                            &Id::Label,
                            Attribute::Text,
                            AttrValue::String(format!("LetterCounter has now value: {}", v))
                        )
                        .is_ok());
                    None
                }
                Msg::LetterCounterBlur => {
                    assert!(self.app.active(&Id::LetterCounter).is_ok());
                    None
                }
                Msg::None => todo!(),
            }
        } else {
            None
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Self {
            app: Model::init_app(),
            quit: false,
            redraw: true,
            terminal: TerminalBridge::new().expect("cannot initialize terminal"),
        }
    }
}
