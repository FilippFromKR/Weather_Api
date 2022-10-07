use std::collections::HashMap;

use
{
    async_trait::async_trait,
    reqwest::Url,
    serde::de::DeserializeOwned,
};

use crate::
{
    error::weather_error::
    {
        ErrorTy, WeatherErr,
    },
    storage::
    {
        {
             Storage,
        },
        repository::Repository,
    },
    utils::types::
    {
        RequestParams, WeatherResult,
    },
    weather::main::
    {
        DependencyHandler,
        Sender,
    },
    service::providers::
    {
        Provide, Providers
    }
};

pub struct RequestSender<S>
{
    providers: HashMap<&'static str, Box<dyn Provide + Send + Sync>>,
    _marker: std::marker::PhantomData<fn() -> S>,
}

#[async_trait]
impl<S: Storage + Send + Sync + 'static> Sender for RequestSender<S>

{
    type Input = &'static str;
    type Error = WeatherErr;

    async fn send_request<T, D>(&self, input: Self::Input, _: Option<&D>) -> WeatherResult<Option<T>>
        where T: DeserializeOwned,
              D: DependencyHandler + Send + Sync,
    {
        /// todo: move to aggregator
    //     let config = dependency.get_dependency::<Repository<S, Providers>>()?
    //         .ok_or(WeatherErr::new(ErrorTy::DEPENDENCY, "Didn't find storage dependency"))?;
    //
    //     let provider_holder = config.get_config()?;
    //
    //     let created_url = self.providers.get(&provider_holder.to_string())
    //         .ok_or(WeatherErr::new(ErrorTy::STORAGE, "Providers not su"))


        let url = Url::parse(input)
            .map_err(|err| WeatherErr::new(ErrorTy::PARSE, err))?;

        let response = reqwest::get(url)
            .await?
            .bytes()
            .await?;

        if response.is_empty()
        {
            Ok(None)
        } else {
            let result = serde_json::from_slice::<T>(response.as_ref()).unwrap();
            Ok(Some(result))
        }
    }
}
