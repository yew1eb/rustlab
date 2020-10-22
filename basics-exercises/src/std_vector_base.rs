#[cfg(test)]
mod test {

    #[test]
    fn base() {
        // 迭代器可以收集到 vector
        let collected_iterator: Vec<i32> = (0..10).collect();
        println!("Collected (0..10) into: {:?}", collected_iterator);

        // `vec!` 宏可用来初始化一个 vector
        let mut xs = vec![1i32, 2, 3];
        println!("Initial vector: {:?}", xs);

        // 在 vector 的尾部插入一个新的元素
        println!("Push 4 into the vector");
        xs.push(4);
        println!("Vector: {:?}", xs);

        // 报错！不可变 vector 不可增长
        //collected_iterator.push(0);
        // 改正 ^ 将此行注释掉

        // `len` 方法获得一个 vector 的当前大小
        println!("Vector size: {}", xs.len());

        // 在中括号上加索引（索引从 0 开始）
        println!("Second element: {}", xs[1]);

        // `pop` 移除 vector 的最后一个元素并将它返回
        println!("Pop last element: {:?}", xs.pop());

        // 超出索引范围将抛出一个 panic
        // println!("Fourth element: {}", xs[3]);
    }

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
            Context::Float(0.001),
        ];

        let v = vec![1, 2, 3, 4, 5];
        let first = &v[0]; //不可变引用
                           //v.push(6); //ERROR, 进行可变修改
        println!("first = {}", first); //禁止再使用不可变引用

        println!("Hello, world!");
    }
}
