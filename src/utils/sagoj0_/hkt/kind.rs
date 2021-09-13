pub mod option;
pub mod result;
pub mod tuple;
pub trait Kind {
    type Inner;
    type Lifted<T>: Kind;
}
