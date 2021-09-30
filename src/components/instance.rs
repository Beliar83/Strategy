use gdnative::{GodotObject, TRef};
use std::marker::PhantomData;

pub struct GodotInstance<T>
where
    T: GodotObject,
{
    pub id: i64,
    instance_type: PhantomData<T>,
}

impl<T> GodotInstance<T>
where
    T: GodotObject,
{
    pub fn new(id: i64) -> GodotInstance<T> {
        GodotInstance::<T> {
            id,
            instance_type: PhantomData::default(),
        }
    }

    pub fn get_instance(&self) -> TRef<'_, T> {
        unsafe { TRef::<T>::from_instance_id(self.id) }
    }
}
