mod garden;
mod tools;

fn main() {
    println!("Hello, world!");
    let g = crate::garden::Gnome;
    println!("Gnome is: {g:?}");
    let t = dbg!(crate::tools::Axe);
    let e = dbg!(crate::garden::enemy::Goblin);
}
