use crate::error::weather_error::WeatherErr;

pub type WeatherResult< T > = Result< T, WeatherErr>;

pub struct RequestParams
{
    address: String,
    data: String,

}


pub struct UserConfig;