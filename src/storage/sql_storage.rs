
use crate::
{
    storage::
    {
        db::Db,
        Storage,
    },
    utils::types::
    {
        UserConfig, WeatherResult,
    },
    service::providers::Providers
};

pub struct SqlLite
{
    db: Db,
}

impl SqlLite
{
    pub fn new(db: Db) -> WeatherResult<SqlLite>
    {
        let db = SqlLite
        {
            db,
        };
        db.connect()?;

        Ok(db)
    }
    fn connect(&self) -> WeatherResult<()>
    {
        let default_provider = Providers::default();
        self.db
            .connection()
            .execute(
                format!("CREATE TABLE IF NOT EXISTS config ( provider VARCHAR(20) NOT NULL ); \
                           INSERT INTO config (provider) VALUES ('{}');", default_provider)
            )?;
        Ok(())
    }
}

impl Storage for SqlLite
{
    type Params = UserConfig;

    fn save(&self, params: Self::Params) -> WeatherResult<()>
    {
        self.db
            .connection()
            .execute(format!("INSERT INTO config (provider) VALUES ('{}')", *params))
            ?;
        Ok(())
    }


    fn get(&self ) -> WeatherResult<Self::Params>
    {
        let mut provider = Default::default();
        self.db
            .connection()
            .iterate("SELECT * FROM config", |config| {
                let result = config.iter()
                    .next();
                if let Some((_, value)) = result
                {
                    // SAVE to unwrap, provider is a NOT NULL field.
                    provider = Providers::try_from(value.unwrap()).unwrap_or_default();
                }
                true
            })
            ?;
        Ok(UserConfig(provider))
    }
}

#[cfg(test)]
// should be run in a single thread
pub mod sql_lite_storage_test
{
    use sqlite::Connection;

    use crate::storage::db::Db;
    use crate::storage::sql_storage::SqlLite;
    use crate::storage::Storage;
    use crate::utils::types::{Providers, UserConfig, WeatherResult};

    fn creat_connection() -> WeatherResult<SqlLite>
    {
        let db = Db::build("./db.config").unwrap();
        SqlLite::new(db)
    }

    #[test]
    fn correct_table_creating()
    {
        let storage = creat_connection();

        assert!(storage.is_ok());
    }

    #[test]
    #[ignore]
    fn correct_get()
    {
        let storage = creat_connection().unwrap();
        let result = storage.get(None);
        assert!(result.is_ok());
    }

    #[test]
    #[ignore]
    fn correct_save()
    {
        let storage = creat_connection().unwrap();
        let result = storage.save(UserConfig(Providers::Open_W));
        assert!(result.is_ok());
        let result = storage.get(None);
        assert!(result.is_ok());
        assert_eq!(&*result.unwrap().to_string(), "open_w");
    }
}