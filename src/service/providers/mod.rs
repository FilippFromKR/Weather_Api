pub mod open_weather;
pub mod weather_a;

use
{
    async_trait::async_trait,
    std::fmt::
    {
        Display, Formatter
    }
};
use crate::
{
    utils::types::
    {
        RequestParams, WeatherResult,

    },
    error::weather_error::
    {
        ErrorTy, WeatherErr
    }
};

#[async_trait]
pub trait Provide
{
    fn url(&self) -> &'static str;
    async fn build_request(&self, params: RequestParams) -> WeatherResult<String>;
    fn short_name(&self) -> &'static str;
}

#[derive(Debug)]
pub enum Providers
{
    #[allow(non_camel_case_types)]
    Open_W,
    #[allow(non_camel_case_types)]
    Api_W,
}



impl<'a> TryFrom<&'a str> for Providers
{
    type Error = WeatherErr;

    fn try_from(value: &str) -> Result<Self, Self::Error>
    {
        match value
        {
            _ if  value == "open_w" => Ok(Providers::Open_W),
            _ if  value == "api_w" => Ok(Providers::Api_W),
            _ => Err(WeatherErr::new(ErrorTy::PARSE, "Wrong argument to get Provider."))
        }
    }
}

impl Display for Providers {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl Default for Providers {
    fn default() -> Self {
        Providers::Api_W
    }
}