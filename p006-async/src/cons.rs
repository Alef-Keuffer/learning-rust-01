#[derive(Debug)]
pub enum List<T, Q> {
    Cons(T, Q),
}
#[derive(Debug)]
pub struct Nil;
