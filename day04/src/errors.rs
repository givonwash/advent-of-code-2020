/// Errors encountered when parsing RGB Hex
#[derive(Clone)]
pub(crate) enum ParseRgbError {
    ValueTooLarge,
    TooManyDigits,
    TooFewDigits,
    InvalidChars,
    MissingHashtag,
    NoValue,
}

/// Errors encountered when parsing height
#[derive(Clone)]
pub(crate) enum ParseHeightError {
    NoValue,
    InvalidValue,
    InvalidUnit,
    NoUnit,
}

/// Errors encountered when parsing eye color
#[derive(Clone)]
pub(crate) enum ParseEyeColorError {
    NoValue,
    InvalidValue,
}

#[derive(Clone)]
pub(crate) enum ParsePassportIntError {
    TooFewDigits,
    TooManyDigits,
    InvalidChars,
    NoValue,
}

/// Errors encountered when parsing a passport
#[derive(Clone)]
pub(crate) enum ParsePassportError {
    UnknownKey,
}

/// Errors encountered when checking a passport
#[derive(Clone)]
pub(crate) enum CheckPassportError<E> {
    ParsingError(E),
    LogicError(PassportInvalidLogicError),
    DoesNotExist,
}

/// Logical errors with a passport
#[derive(Clone)]
pub(crate) enum PassportInvalidLogicError {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
}
