trait B{
    fn print(&self);
    fn set(&mut self, value:i32);
    fn new()->Self;
}

struct C{
    val: i32
}

impl B for C{
    fn print(&self){
        println!("{}", self.val);
    }
    fn set(&mut self, value:i32){
        self.val = value;
    }
    fn new() -> Self{
        C {val:0}
    }
}

fn main() {
    let mut a = C::new();
    a.set(20);
    a.print();
}