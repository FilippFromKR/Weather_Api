use
{
    async_trait::async_trait,
    serde::de::DeserializeOwned,
    std::
    {
        any::Any,
        fmt::Debug,
        ops::Deref,
        sync::Arc,

    },
};
use crate::utils::types::WeatherResult;

#[async_trait]
pub trait WeatherAggregator<S: Sender, R: Receiver, D: DependencyHandler>
{
    type Output;

    async fn forecast(&self) -> Self::Output;
}

pub trait DependencyHandler
{

    fn add_dependency<T>(&mut self, dependency: T) -> WeatherResult<()>
        where T: Any + 'static;

    fn get_dependency<T>(&self) -> WeatherResult<Option<DepHolder<T>>>
        where T: 'static;
}

#[derive(Debug)]
pub struct DepHolder<T>(Arc<Box<T>>);

impl<T> DepHolder<T> {
    pub fn new(t: Arc<Box<T>>)->DepHolder<T>
    {
        Self(t)
    }

}
impl<T> Deref for DepHolder<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}


pub trait Request<C, O>
{
    fn create_request(&self, config: C)
                      -> O;
}

#[async_trait]
pub trait Sender
{
    type Input;
    type Error;

    async fn send_request<T, D>(&self, input: Self::Input, dependency: Option<&D>) -> Result<Option<T>, Self::Error>
        where T: DeserializeOwned,
              D: DependencyHandler + Send + Sync;
}


pub trait Receiver
{
    type Config;
    type Output;
    fn handle_input(&self, dependency: Self::Config) -> Self::Output;
}

