use sqlite::Connection;

use crate::
{
    error::weather_error::
    {
        ErrorTy, WeatherErr
    },
    utils::types::WeatherResult
};

pub struct Storage
{
    connection: Connection,
}

impl Storage {
    pub fn build(path: &str) -> WeatherResult<Self>
    {
        let connection = sqlite::open(path)
            .map_err(|err| WeatherErr::new(ErrorTy::STORAGE, err))?;
        Ok
            (
                Self
                {
                    connection
                }
            )
    }
}