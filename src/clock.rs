pub trait Clock {
    fn elapsed_since_start(&self) -> u128;
    fn sleep(&self);
}
