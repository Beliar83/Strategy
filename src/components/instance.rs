use gdnative::prelude::*;
use gdnative::{GodotObject, Ref, TRef};
use std::marker::PhantomData;

pub struct GodotInstance<T>
where
    T: GodotObject<RefKind = ManuallyManaged>,
{
    node: Ref<T, Shared>,
    instance_type: PhantomData<T>,
}

impl<T> GodotInstance<T>
where
    T: GodotObject<RefKind = ManuallyManaged>,
{
    pub fn new(node: Ref<T, Shared>) -> GodotInstance<T> {
        GodotInstance::<T> {
            node,
            instance_type: PhantomData::default(),
        }
    }

    pub fn get_node_if_sane(&self) -> Option<TRef<'_, T, Shared>> {
        unsafe { self.node.assume_safe_if_sane() }
    }

    pub fn get_node(&self) -> TRef<'_, T, Shared> {
        unsafe { self.node.assume_safe() }
    }

    pub fn cast<T2>(&self) -> Option<TRef<'_, T2, Shared>>
    where
        T2: GodotObject<RefKind = ManuallyManaged> + SubClass<T>,
    {
        self.get_node().cast::<T2>()
    }

    pub fn cast_instance<T2>(&self) -> Option<RefInstance<'_, T2, Shared>>
    where
        T2: NativeClass<Base = T>,
    {
        self.get_node().cast_instance::<T2>()
    }
}
