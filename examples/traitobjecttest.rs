
trait Printable  {
    fn stringfiy(&self)->String;
}

impl Printable for i32 { 
    fn stringfiy(&self)->String {
        self.to_string()
    }
}

fn print( a : Box<dyn Printable>) {
    println!("{}",a.stringfiy());
}

fn main() {

    print(Box::new(10) as Box<dyn Printable>);

}