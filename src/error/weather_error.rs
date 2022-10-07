use serde_json::Error;
use
{
    std::
    {
        fmt::Display,
        sync::
        {
            PoisonError
        },
    }
};

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

impl From<sqlite::Error> for WeatherErr
{
    fn from(err: sqlite::Error) -> Self
    {
        WeatherErr::new(ErrorTy::STORAGE, err)
    }
}

impl From<reqwest::Error> for WeatherErr
{
    fn from(err: reqwest::Error) -> Self
    {
        WeatherErr::new(ErrorTy::SERVICE, err)
    }
}

impl From<serde_json::Error> for WeatherErr
{
    fn from(err: Error) -> Self
    {
        WeatherErr::new(ErrorTy::PARSE, err)
    }
}