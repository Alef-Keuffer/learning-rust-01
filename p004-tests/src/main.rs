use std::thread;
use std::cell::Cell;
use std::ptr;
use std::ops::Deref;

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &Box::new(&self.0)
    }
}

#[derive(Debug)]
enum List<T, Q> {
    Cons(T, Q),
}
#[derive(Debug)]
struct Nil;
use List::{Cons};

fn main() {
    let v = vec![1, 2, 3, 4];
    let a: Vec<_> = v.iter().filter(|&&x| x % 2 == 0).map(|x: &i32| x * 2).collect();
    let b: Vec<_> = v.iter().map(|x| x * 2).filter(|x: &i32| x % 2 == 0).collect();
    println!("{} {}", a[0], b[0]);

    let list = Cons("a", Cons(2, Nil));
    println!("{list:?}");
    println!("{}", std::mem::size_of::<i128>());
    assert_eq!(32,std::mem::size_of::<List<i128, List<bool, List<bool,()>>>>());
    assert_eq!(3,std::mem::size_of::<List<bool, List<bool, List<bool,()>>>>());
    assert_eq!(48,std::mem::size_of::<List<bool, List<bool, List<i128,()>>>>());
    assert_eq!(16,std::mem::size_of::<i128>());

    let x = 5;
    let y = &x;
    let z = Box::new(x);
    let a: *const i32 = &1;
    let ay: *const i32 = y;

    let my_box = MyBox::new(11);

    dbg!(x);
    dbg!(y);
    dbg!(*y);
    dbg!(&y);
    dbg!(&x==y);
    dbg!(&5==y);
    dbg!(&*y==&*z);
    dbg!(&*z==&5);
    dbg!(&5);
    dbg!(&5 + 6);
    dbg!(5 + &6);
    dbg!(&5 + &6);
    dbg!(&(5+6));
    dbg!(&(5+6) == &11);
    dbg!(ptr::eq(&5,y));
    dbg!(ptr::eq(&5,&5));
    dbg!(&a);
    dbg!(a);
    dbg!(ptr::eq(y,ay));
    dbg!(*my_box == 11);
    dbg!(&*my_box == &11);
}

fn md<T>(_: T){}
