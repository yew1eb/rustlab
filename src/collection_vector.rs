
#[cfg(test)]
mod test {
    #[test]
    fn tmain() {
        let mut v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    {
        let v1 = vec![1, 2, 3];
    }

    let one: &i32 = &v[0];
    println!("one = {}", *one);

    match v.get(1) {
        Some(value) => println!("value = {}", value),
        _ => println!("None"),
    }

    let mut v2: Vec<i32> = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);

    for i in &v2 {
        println!("i = {}", i);
    }

    for i in &mut v2 {
        *i += 1;
        println!("i = {}", i);
    }

    enum Context {
        Text(String),
        Float(f32),
        Int(i32),
    };

    let c = vec![
        Context::Text(String::from("string")),
        Context::Int(-1),
        Context::Float(0.001)
    ];


    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; //不可变引用
    //v.push(6); //ERROR, 进行可变修改
    println!("first = {}", first); //禁止再使用不可变引用

    println!("Hello, world!");
    }
}