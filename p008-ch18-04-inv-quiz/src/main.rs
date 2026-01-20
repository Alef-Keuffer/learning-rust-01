use std::fmt::Display;
fn add_displayable<'a, T: Display + 'a>(
    v: &mut Vec<Box<dyn Display + 'a>>,
    t: T
) {
    v.push(Box::new(t));
}

fn main() {

    let mut v: Vec<Box<dyn Display>> = Vec::new();
    {
        let s = String::from("Hello world");
        add_displayable(&mut v, s);
    }
    println!("{}", v[0]);
}
