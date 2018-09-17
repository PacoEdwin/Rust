use std::fmt;
struct A<T>
{
    x: T
}
impl <T> A<T> where T: P {
    fn item(&self){
        &self.x.print();
    }
}
struct B<T>{
    val: T
}
trait P{
    fn print(&self);
}
impl<T: fmt::Display> P for B<T>{
    fn print(&self){
        println!("{}", self.val);
    }
}


fn main(){

    let b = B{val: String::from("zui")};
    let a = A{x:b};
    a.item();
    //let r = a.item().print();
}
