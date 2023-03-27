pub mod animation_driver;
pub mod animation_event;
pub trait Animation<T> {
    fn tick(&mut self, dt: f64);
    fn value(&self) -> T;
}

pub trait Animatable<T, U> {
    fn animate(&self, animation: &dyn Animation<U>) -> T;
}
