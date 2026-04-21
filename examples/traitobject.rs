trait Printable {
    fn stringify(&self) -> String;
}

impl Printable for i32 {
    fn stringify(&self) -> String { self.to_string() }
}

fn print(a: &dyn Printable) {
    println!("{}", a.stringify());
}

fn main() {
    let x = 10;
    print(&x)
}