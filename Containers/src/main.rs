use std::collections::HashMap;
use std::fs::File;
//for errors
enum Result<T,E>{
    Ok(T),
    Err(E)
}

fn main() {

    
    /////////////////////////////////////////////////////////
    let f = File::open("text.txt");
    let f = match f{
        Ok(file) => file,
        Err(error) =>{
            panic!("Something wrong with file!")
        }
    };
    /////////////////////////////////////////////////////////
    
    /////////////////////////////////////////////////////////
    let mut hm = HashMap::new();
    hm.insert(String::from("random"), 4);
    hm.insert(String::from("strings"), 20);
    for (k,v) in &hm{
        println!("{} {}", k,v);
    }
    hm.remove(&String::from("random"));
    match hm.get(&String::from("random")){
        Some(&n) => println!("this is Some: {}", n),
        _ => println!("no match")
    }
    /////////////////////////////////////////////////////////
    
    
    /////////////////////////////////////////////////////////
    let s = Some("c");
    match s{
        Some(i) => println!("{}", i),
        _ =>{}
    }

    if let Some(i) = s{
        println!("{}", i);
    } else{
        {}
    }
    /////////////////////////////////////////////////////////
    

    /////////////////////////////////////////////////////////
    let mut v: Vec<i32>= Vec::new();
    for i in 0..5 {
        v.push(i);
    }  
    for i in &v{
        println!("{}", i); 
    }  
    v.push(10);
    println!("{:?} {} {}", &v, v.len(), v.capacity());
    /////////////////////////////////////////////////////////


}
