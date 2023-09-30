use crate::DatabaseState;

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    InfoLabelChanged(DatabaseState),
    Resize,
    None,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Id {
    InfoLabel,
    Menu,
}