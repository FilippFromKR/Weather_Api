use
{
    serde::de::DeserializeOwned,
    api::weather::main::
    {
        Sender,
        DependencyHandler
    },
    async_trait::async_trait,
    reqwest::Url,
};

use crate::
{
    error::weather_error::WeatherErr,
    utils::types::
    {
        WeatherResult, RequestParams
    }
};
use crate::utils::types::UserConfig;

pub struct RequestSender
{

}

#[async_trait]
impl  Sender  for RequestSender
{
    type Input = RequestParams;
    type Error = WeatherErr;

    async fn send_request< T,D >(&self, input: Self::Input, dependency: D) -> WeatherResult<Option < T >>
        where T: DeserializeOwned,
        D: DependencyHandler + Send,
    {
        // Should panic if configs didn't add.
        let config = dependency.get_dependency::<UserConfig>()?
            .expect("Can't find storage, to get configuration.");

        let url = Url::parse_with_params()
        let result = reqwest::get(format!("{}{}", url, addition.unwrap_or("")))
            .await?
            .bytes()
            .await?;
        if result.is_empty()
        {
            Ok( None )
        }
        else
        {
            let result = serde_json::from_slice::<T>(result.as_ref()).unwrap();
            Ok( Some( result ) )
        }
        todo!()
    }
}