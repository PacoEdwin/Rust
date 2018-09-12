

// fn lol(a:&mut i32, b:&mut i32){
//     let l = *a+*b;
//     println!{"{}", *a+*b};
// }

// fn take(v: &mut Vec<i32>){
//     v[10] +=v[100];
//     println!("{}", v[10]);
// }

fn main() {
let a = 2;
let b = 3;
let l = &a;
let v = l;
println!("{}",*l);
//println!("{}", l);
// v = count(v, 1)
// println!("{}", v[1]);

// let mut v = Vec::new();
// for i in 0..1000{
//     v.push(i);
// }
//let d = &v;
//println!("{}", *d[1]);
// take(&mut v);
// println!("{}", v[10]);

// let mut a = 32;
// let mut b = 45;
// let c = &mut a;
// let e = &c;
// let mut d = &mut b;
// let f = &mut d;
// **f+=1;
// println!("{}", *f);
// lol(c,*f);
// println!("{} {} ", *c , *d);
/*for i in 0..a.len(){
    a[i]= 1;
    println!("{}", a[i]);
}

for i in 0..a.len(){
    a[i]= 1;
    println!("{}", a[i]);
}

for i in &v {
    println!("A reference to {}", i);
}

for i in &mut v {
    println!("A mutable reference to {}", i);
}

for  i in &mut v {
    *i+=1;
    println!("Take ownership of the vector and its element {}", i);
}
println!("lol: {}", v[2]);
    let mut a:[i32;5]=[1,2,3,4,5];
    for (index, i) in a.iter().enumerate(){
        a[index] = *i as i32;
    }*/
}
