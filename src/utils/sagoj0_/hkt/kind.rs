pub mod option;
pub mod result;

pub trait Kind1 {
    type Inner;
    type Lifted<T>: Kind1;
}

pub trait Kind2 {
    type Inner1;
    type Inner2;
    type Lifted<T, U>: Kind2;
}
