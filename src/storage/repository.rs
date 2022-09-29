use crate::error::weather_error::WeatherErr;
use crate::storage::Storage;
use crate::utils::types::{UserConfig, WeatherResult};


pub struct Repository<S>
{
    storage: S

}
impl<S: Storage> Repository<S>
{
    pub fn get_config<P>(&self,params: Option<P>) -> WeatherResult<UserConfig>
    {
        self.storage.get(params)

    }

    pub fn new_config<P>(&self,params: Option<P>) -> WeatherResult<()>
    {
        self.storage.save(params)
    }
    pub fn update_config<P>(&self,params: Option<P>) -> WeatherResult<()>
    {
        self.storage.update(params)
    }
    pub fn delete_config<P>(&self,params: Option<P>) -> WeatherResult<()>
    {
        self.storage.delete(params)
    }
}