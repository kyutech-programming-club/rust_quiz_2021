pub mod option;
pub mod result;

pub trait Kind1 {
    type Inner;
    type Lifted<T>: Kind1;
}
