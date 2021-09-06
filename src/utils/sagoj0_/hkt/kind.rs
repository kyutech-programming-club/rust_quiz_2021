pub mod func;
pub mod option;
pub mod result;
pub trait Kind1 {
    type Inner;
    type Lifted<T>: Kind1;
}

pub trait Kind2: Kind1 {
    type Inner1;
    type Kind1<T>;
    type Lifted<T, U>: Kind2;
}
