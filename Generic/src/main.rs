use std::fmt;
struct A<T>
{
    x: T
}
impl <T> A<T>{
    fn item(&self) ->&T{
        &self.x
    }
}
struct B{
    val: i32
}
impl B{
    fn print(&self){
        println!("{}", self.val);
    }
}
fn main(){

    let b = B{val: 20};
    let a = A{x:b};
    let r = a.item().print();
}
