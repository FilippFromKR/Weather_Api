use async_trait::async_trait;
use crate::
{
    service::providers::Provide,
    utils::types::
    {
        RequestParams, WeatherResult, ForecastType
    }
};
pub struct WeatherApi
{
    app_id: String
}

impl WeatherApi {
    fn new(app_id: String) -> Self
    {
        Self
        {
            app_id
        }
    }

}

#[async_trait]
impl Provide for WeatherApi
{
    fn url(&self) -> &'static str
    {
        "https://api.weatherapi.com/v1/"
    }

    async fn build_request(&self, params: RequestParams) -> WeatherResult<String>
    {

       let final_url =  if let ForecastType::Daily(days) = params.data
        {
            format!("{}/forecast.json?key={}&q={}&days={}",self.url(),self.app_id,params.city,days)
        }
        else
        {
            format!("{}/current.json?key={}&q={}",self.url(),self.app_id,params.city)
        };
        Ok(final_url)

    }

    fn short_name(&self) -> &'static str {
        "api_w"
    }
}