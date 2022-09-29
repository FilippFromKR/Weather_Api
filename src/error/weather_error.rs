use std::borrow::Cow;
use std::fmt::Display;
use std::sync::{LockResult, PoisonError};

use sqlite::Error;

#[derive(Debug)]
pub struct WeatherErr
{
    message: String,
    ty: ErrorTy,
}

impl WeatherErr
{
    pub fn new<T>(ty: ErrorTy, message: T) -> Self
        where T: Display
    {
        Self
        {
            message: message.to_string(),
            ty,
        }
    }
}

#[derive(Debug)]
pub enum ErrorTy
{
    USER,
    SERVICE,
    DEPENDENCY,
    PARSE,
    STORAGE,
    POISONED,
}

impl<G> From<PoisonError<G>> for WeatherErr
{
    fn from(err: PoisonError<G>) -> Self
    {
        WeatherErr::new(ErrorTy::POISONED, err)
    }
}

impl<E> From<sqlite::Error> for WeatherErr
{
    fn from(err: Error) -> Self
    {
        WeatherErr::new(ErrorTy::STORAGE, err)
    }
}