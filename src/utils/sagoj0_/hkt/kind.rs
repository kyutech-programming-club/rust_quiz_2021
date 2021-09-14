pub mod option;
pub mod result;
pub trait Kind {
    type Inner;
    type Lifted<T>: Kind;
}
