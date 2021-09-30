use gdnative::api::Physics2DDirectSpaceState;
use gdnative::TRef;

#[derive(Clone)]
pub struct PhysicsState {
    id: i64,
}
impl PhysicsState {
    pub fn new(id: i64) -> PhysicsState {
        PhysicsState { id }
    }

    pub fn get_instance(&self) -> TRef<'_, Physics2DDirectSpaceState> {
        unsafe { TRef::<Physics2DDirectSpaceState>::from_instance_id(self.id) }
    }
}
