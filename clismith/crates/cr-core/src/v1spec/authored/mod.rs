pub mod core;
pub mod module;
pub mod param;

pub trait Compiled<T> {
    fn to_compiled(&self) -> T;
}
