use crate::utils::types::WeatherResult;

pub mod db;
pub mod repository;
pub mod sql_storage;

pub trait Storage
{
    fn save<P>(&self, params: Option<P>) -> WeatherResult<()>;
    fn update<P>(&self, params: Option<P>) -> WeatherResult<()>;
    fn delete<P>(&self, params: Option<P>) -> WeatherResult<()>;
    fn get<P, R>(&self, params: Option<P>) -> WeatherResult<R>;
}