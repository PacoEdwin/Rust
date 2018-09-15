
struct A{
    val: i32
}

impl A {
    fn print(&self){
        println!("{}", self.val);
    }
    fn new(v : i32) -> A{
       return A{val:v};
    }
}

fn main() {
    let a = A::new(1);
    a.print();
}
