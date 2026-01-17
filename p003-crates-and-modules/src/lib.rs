mod garden;
mod tools;

pub fn print_gnome() {
    let g = crate::garden::Gnome;
    println!("{g:?}");
    let t = crate::tools::Axe;
    println!("My tool is: {t:?}");
}
