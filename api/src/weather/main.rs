use std::ops::Deref;
use std::sync::Arc;

use
{
    async_trait::async_trait,
    serde::de::DeserializeOwned,
    std::any::Any,
};

#[async_trait]
pub trait WeatherAggregator<S: Sender, R: Receiver, D: DependencyHandler>
{
    type Output;

    async fn forecast(&self) -> Self::Output;
}

pub trait DependencyHandler
{
    type Error;

    fn add_dependency<T>(&mut self, dependency: T) -> Result<(), Self::Error>
        where T: Any + 'static;

    fn get_dependency<T>(&self) -> Result<Option<DepHolder<T>>, Self::Error>
        where T: 'static;
}

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

    async fn send_request<T, D>(&self, input: Self::Input, dependency: D) -> Result<Option<T>, Self::Error>
        where T: DeserializeOwned,
              D: DependencyHandler + Send;
}


pub trait Receiver
{
    type Config;
    type Output;
    fn handle_input(&self, dependency: Self::Config) -> Self::Output;
}

