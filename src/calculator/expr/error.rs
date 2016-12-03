use std::fmt;
use std::error::{self, Error};

///////////////////////////////////////////
//  Aux Macro for creating the error type
///////////////////////////////////////////
macro_rules! new_error_type {
    ($Error: ident) => (
        new_error_type!($Error, stringify!($Error));
    );

    ($Error: ident, $Description: expr) => (
        #[derive(Debug)]
        pub struct $Error;
        impl error::Error for $Error {
            fn description(&self) -> &str {
                $Description
            }
        }
        impl_display_for_error!($Error);
    );
}

macro_rules! impl_display_for_error {
    ($Error: ty) => (
        impl fmt::Display for $Error {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.description().fmt(f)
            }
        }
    );
}

#[derive(Debug)]
pub struct EvalError {
    pub kind: EvalErrorKind,
}

#[derive(Debug)]
pub enum EvalErrorKind {
    DivideByZero,
    Overflow,
}


impl error::Error for EvalError {
    fn description(&self) -> &str {
        match self.kind {
            EvalErrorKind::DivideByZero => "cannot divide by zero",
            EvalErrorKind::Overflow => "evaluation overflow",
        }
    }
}


impl_display_for_error!(EvalError);