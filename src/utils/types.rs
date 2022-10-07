use
{
    std::
    {
        fmt::
        {
            Display, Formatter,
        },
        ops::Deref,
        collections::HashMap,
    }
};

use crate::
{
    error::weather_error::
    {
        ErrorTy, WeatherErr,
    },
    service::providers::
    {
        Provide, Providers
    }
};

pub type WeatherResult<T> = Result<T, WeatherErr>;

pub struct RequestParams
{
    pub city: String,
    pub country_code: String,
    pub data: ForecastType,

}

pub enum ForecastType
{
    Daily(u8),
    Now,
}


#[derive(Debug)]
pub struct UserConfig(pub Providers);


impl Deref for UserConfig
{
    type Target = Providers;

    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}












