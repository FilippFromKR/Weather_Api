use crate::service::providers::Provide;
use crate::utils::types::{RequestParams, WeatherResult};

pub mod db;
pub mod repository;
pub mod sql_storage;

pub trait Storage
{
    type Params: Send + Sync ;
    fn save(&self, params: Self::Params) -> WeatherResult<()>;
    fn get(&self) -> WeatherResult<Self::Params>;
}

