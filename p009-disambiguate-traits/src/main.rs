use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {
}

fn bar() -> ! {
    print!("forever ");

    loop {
        print!("and ever ");
    }
}

fn foo() -> ! {
    panic!()
}



fn is_equal<T: Eq + ?Sized>(t1: &T, t2: &T) -> bool {
  t1 == t2
}

fn returns_closure() -> dyn Fn(i32) -> i32 {
   |x| x + 1
}

fn main() {
    let p = Point {x: 1, y:2};
    p.outline_print();
    println!("{}", is_equal("Hello", "world"));
}
