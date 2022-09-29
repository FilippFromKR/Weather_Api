use
{
    api::weather::main::
    {
        DependencyHandler,
        DepHolder
    },
    std::
    {
        any::
        {
            Any, TypeId,
        },
        borrow::Borrow,
        borrow::BorrowMut,
        collections::HashMap,
        sync::
        {
            Arc, Mutex,
        },
    },
};

use crate::
{
    error::weather_error::
    {
        ErrorTy, WeatherErr
    },
    utils::types::WeatherResult
};

pub struct DependencyBuilder
{
    deps: Arc<Mutex<HashMap<TypeId, Arc<Box<dyn Any + 'static>>>>>,
}

impl DependencyBuilder
{
    pub fn new() -> DependencyBuilder
    {
        Self
        {
            deps: Arc::new(Mutex::new(Default::default()))
        }

    }
    fn ty_id<T>(&self) -> TypeId
        where T: 'static
    {
        TypeId::of::<T>()
    }
}

impl DependencyHandler for DependencyBuilder
{
    type Error = WeatherErr;

    /// Add dependency to hash_map with key TypeID, so may keep only unique structs.
    fn add_dependency<T>(&mut self, dependency: T) -> WeatherResult<()>
        where T: Any + 'static
    {
        let ty_id = self.ty_id::<T>();
        let mut lock = self.deps.lock()?;
        if lock.contains_key(&ty_id)
        {
            return Err(WeatherErr::new(ErrorTy::DEPENDENCY,
                                       "Dependency Handler can have only unique structs. "));
        }


        lock.insert(ty_id, Arc::new(Box::new(dependency)));
        Ok(())
    }

    /// Get dependency by TypeID
    fn get_dependency<T>(&self) -> WeatherResult<Option<DepHolder<T>>>
        where T: 'static
    {
        let borrowed: &Mutex<HashMap<TypeId, Arc<Box<dyn Any + 'static>>>> = self.deps.borrow();
        let ty_id = self.ty_id::<T>();
        let result = Arc::clone(borrowed.lock()?.get(&ty_id).unwrap());

        // SAFETY: We know that we store T type, so it's save to get T.
        let as_t: Arc<Box<T>> = unsafe
            {
                std::mem::transmute::<Arc<Box<dyn Any>>, Arc<Box<T>>>(result)
            };

        Ok
            (
                Some
                    (
                        DepHolder::new(as_t)
                    )
            )
    }
}


#[cfg(test)]
mod dependency_handler_check
{
    use std::sync::{Arc, Mutex};

    use api::weather::main::DependencyHandler;

    use crate::dependencies::builder::DependencyBuilder;

    struct Test1;

    struct Test2;

    struct Test3;

    #[test]
    fn correctly_insert_unique()
    {
        let mut d_b = DependencyBuilder { deps: Arc::new(Mutex::new(Default::default())) };
        d_b.add_dependency(Test1);
        d_b.add_dependency(Test2);
        d_b.add_dependency(Test3);
        let result1 = d_b.get_dependency::<Test1>();
        let result2 = d_b.get_dependency::<Test2>();
        let result3 = d_b.get_dependency::<Test3>();
        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_ok());
    }

    #[test]
    fn reject_not_unique()
    {
        let mut d_b = DependencyBuilder { deps: Arc::new(Mutex::new(Default::default())) };
        d_b.add_dependency(Test1);
        let error = d_b.add_dependency(Test1);
        assert!(error.is_err());
        let first_added_dp = d_b.get_dependency::<Test1>();

       assert!(first_added_dp.is_ok())
    }
}

