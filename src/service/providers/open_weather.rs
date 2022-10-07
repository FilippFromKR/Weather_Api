use {
    std::collections::HashMap,
    async_trait::async_trait
};
use crate::
{
    error::weather_error::
    {
        ErrorTy, WeatherErr
    },
    service::providers::Provide,
    utils::types::
    {
        ForecastType, RequestParams, WeatherResult
    }
};

pub struct OpenWeather
{
    app_id: String,
}


impl OpenWeather {
    fn new(app_id: String) -> Self
    {
     Self
     {
         app_id
     }
    }
    async fn lat_lon(&self, city: &str, county_code: &str) -> WeatherResult<(f64, f64)>
    {
        let result = reqwest::get(format!("https://api.openweathermap.org/geo/1.0/direct?q={}{}&limit=1&appid={}", city.to_lowercase(), county_code, self.app_id))
            .await?
            .bytes()
            .await?;
        if result.is_empty()
        {
            return Err(WeatherErr::new(ErrorTy::SERVICE, "Fail to get coordinates."));
        }
        let response_map = serde_json::from_slice::<Vec<HashMap<String, f64>>>(&result)?;

        // We checked, that response is not empty.
        let map = response_map.get(0).unwrap();
        let lat = *map.get("lat").unwrap();
        let lon = *map.get("lot").unwrap();
        Ok((lat, lon))
    }
}


#[async_trait]
impl Provide for OpenWeather
{
    fn url(&self) -> &'static str
    {
        "https://api.openweathermap.org/data/2.5/"
    }
   async fn build_request(&self, params: RequestParams) -> WeatherResult<String>
    {
        let (lat, lon) = self.lat_lon(&params.city, &params.country_code).await?;
        let mut final_url = self.url();

        let final_url = if let ForecastType::Daily(days) = params.data
        {
            format!("{}/forecast/daily?/lat={}&lon={}&cnt={}&units=metric&appid={}", final_url, lat, lon, days, self.app_id)
        } else {
            format!("{}/weather?/lat={}&lon={}&units=metric&appid={}", final_url, lat, lon, self.app_id)
        };
        Ok(final_url)
    }


    fn short_name(&self) -> &'static str {
        "open_w"
    }
}