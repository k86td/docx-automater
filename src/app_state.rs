use std::time::Duration;

use crate::{
    components::{container::Container, label::Label},
    messages::{Id, Msg},
    DatabaseState,
};
use tuirealm::{
    props::{Alignment, BorderType, Borders, Color, Style, TextModifiers},
    terminal::TerminalBridge,
    tui::{
        prelude::{Constraint, Direction, Layout},
        style::Stylize,
    },
    Application, AttrValue, Attribute, EventListenerCfg, NoUserEvent, Update,
};

pub struct AppState {
    pub app: Application<Id, Msg, NoUserEvent>,
    pub quit: bool,
    pub redraw: bool,
    pub terminal: TerminalBridge,

    pub database_state: DatabaseState,
}

impl AppState {
    pub fn view(&mut self) {
        assert!(self
            .terminal
            .raw_mut()
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Length(1), Constraint::Percentage(100)].as_ref())
                    .split(f.size());

                self.app.view(&Id::InfoLabel, f, chunks[0]);
                self.app.view(&Id::Menu, f, chunks[1]);

                _ = self.app.active(&Id::Menu);
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
                Id::InfoLabel,
                Box::new(Label::default().text("> db:?").foreground(Color::Gray)),
                Vec::default()
            )
            .is_ok());

        assert!(app
            .mount(
                Id::Menu,
                Box::new(
                    Container::default()
                        .borders(Borders::default().modifiers(BorderType::Rounded))
                        .title(" Menu ", Alignment::Center)
                ),
                Vec::default()
            )
            .is_ok());

        app
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            app: AppState::init_app(),
            quit: false,
            redraw: true,
            terminal: TerminalBridge::new().expect("can initialize terminal"),
            database_state: DatabaseState::Unknown,
        }
    }
}

impl Update<Msg> for AppState {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        if let Some(msg) = msg {
            self.redraw = true;

            match msg {
                Msg::AppClose => {
                    self.quit = true;
                    None
                }
                Msg::InfoLabelChanged(s) => {
                    let msg: String = match s {
                        DatabaseState::Created => {
                            assert!(self
                                .app
                                .attr(
                                    &Id::InfoLabel,
                                    Attribute::Foreground,
                                    AttrValue::Color(Color::Yellow),
                                )
                                .is_ok());
                            "> db:created".to_string()
                        }
                        DatabaseState::Connected => {
                            assert!(self
                                .app
                                .attr(
                                    &Id::InfoLabel,
                                    Attribute::Foreground,
                                    AttrValue::Color(Color::Green),
                                )
                                .is_ok());
                            "> db:connected".to_string()
                        }
                        _ => {
                            assert!(self
                                .app
                                .attr(
                                    &Id::InfoLabel,
                                    Attribute::Foreground,
                                    AttrValue::Color(Color::Gray),
                                )
                                .is_ok());
                            "> db:?".to_string()
                        }
                    };

                    assert!(self
                        .app
                        .attr(&Id::InfoLabel, Attribute::Text, AttrValue::String(msg))
                        .is_ok());
                    None
                }
                Msg::Resize => {
                    self.redraw = true;
                    None
                }
                _ => None,
            }
        } else {
            None
        }
    }
}
