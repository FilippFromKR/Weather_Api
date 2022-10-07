use
std::marker::PhantomData;

use crate::
{
    storage::Storage,
    utils::types::
    {
        WeatherResult,
    }
};

pub struct Repository<S, C>
{
    storage: S,
    _marker: PhantomData<C>,

}

impl<C, S: Storage<Params = C> > Repository<S, C>
{
    pub fn get_config(&self) -> WeatherResult<C>
    {
        self.storage.get()
    }

    pub fn new_config(&self, params: C) -> WeatherResult<()>
    {
        self.storage.save(params)
    }
}
