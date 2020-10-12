fn main() {
    let mut s0 = String::new();
    s0.push_str("hello");
    println!("s0 = {}", s0);
    s0.push_str(", world");
    println!("{}", s0);

    let s1 = String::from("init some thing");
    println!("{}", s1);

    let s1 = "init some thing".to_string();
    println!("{}", s1);

    let mut s2 = String::from("hello");
    s2.push_str(", world");
    let ss = " !".to_string();
    s2.push_str(&ss);
    println!("{}", s2);
    println!("ss = {}", ss);


    let mut s2 = String::from("tea");
    s2.push('m');// push only request one char
    println!("{}", s2);

    let s1 = "hello".to_string();
    let s2 = String::from(", world");
    let s3 = s1 + &s2; // +操作实际上是s1(String)的方法
    println!("s3 = {}", s3);
    println!("s3.len = {}", s3.len());
    //println!("s1 = {}", s1); //ERROR value borrowed here after move
    println!("s2 = {}", s2);


    let hello = "你好";
    let h5 = &hello[0..3];
    println!("h5 = {}", h5);

    //let h6 = &hello[0..2];
    //println!("h6 = {}", h6); //panicked at 'byte index 2 is not a char boundary;


    //chars
    for c in "你好".to_string().chars() {
        println!("c = {}", c);
    }

    //bytes
    for b in "hello, world".to_string().bytes()  {
        println!("b = {}", b);
    }


    let country = return_str();//ERROR:  this function's return type contains a borrowed value, but there is no value for it to be borrowed from

}


//the function return the reference. But the String country only lives inside the function, and then it dies.
//Once a variable is gone, the computer will clean up the memory and use it for something else. 
//So after the function is over, country_ref is referring to memory that is already gone.
fn return_str() -> &str {
    let country = String::from("SA");
    let country_ref = &country;
    country_ref
}
