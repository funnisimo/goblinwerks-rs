// use super::DenseStorage;

pub trait Component: Sized + 'static {}

impl<T> Component for T where T: Sized + 'static {}
