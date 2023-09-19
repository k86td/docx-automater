#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    DigitCounterChanged(isize),
    DigitCounterBlur,
    LetterCounterChanged(isize),
    LetterCounterBlur,
    None,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Id {
    DigitCounter,
    LetterCounter,
    Label,
}