use
sqlite::Connection;
use crate::storage::Storage;

use crate::utils::types::WeatherResult;

pub struct SqlLite
{
    db: Connection,
}

impl SqlLite {
    pub fn new(db: Connection) -> SqlLite
    {
        let db = Self
        {
            db
        };
        db.connect()?;
        db
    }
    fn connect(&self) -> WeatherResult<()>
    {
        self.db
            .execute(
                "CREATE TABLE config IF NOT EXIST (config JSON); "
            ).into()
    }
}
impl Storage for SqlLite
{
    fn save<P>(&self, params: Option<P>) -> WeatherResult<()> {
        todo!()
    }

    fn update<P>(&self, params: Option<P>) -> WeatherResult<()> {
        todo!()
    }

    fn delete<P>(&self, params: Option<P>) -> WeatherResult<()> {
        todo!()
    }

    fn get<P, R>(&self, params: Option<P>) -> WeatherResult<R> {
        todo!()
    }
}