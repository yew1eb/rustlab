#[cfg(test)]
mod test {

    #[test]
    fn base() {
        // （所有的类型标注都是都是多余）
        // 一个指向在只读内存中堆分配字符串的引用
        let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
        println!("Pangram: {}", pangram);

        // 逆序迭代单词，不用分配新的字符串
        // （原文：Iterate over words in reverse, no new string is allocated）
        println!("Words in reverse");
        for word in pangram.split_whitespace().rev() {
            println!("> {}", word);
        }

        // 复制字符到一个 vector，排序并移除重复值
        let mut chars: Vec<char> = pangram.chars().collect();
        chars.sort();
        chars.dedup();

        // 创建一个空的且可增长的 `String`
        let mut string = String::new();
        for c in chars {
            // 在字符串的尾部插入一个字符
            string.push(c);
            // 在字符串尾部插入一个字符串
            string.push_str(", ");
        }

        // 此切割的字符串是原字符串的一个切片，所以没有执行新分配操作
        let chars_to_trim: &[char] = &[' ', ','];
        let trimmed_str: &str = string.trim_matches(chars_to_trim);
        println!("Used characters: {}", trimmed_str);

        // 堆分配一个字符串
        let alice = String::from("I like dogs");
        // 分配新内存并存储修改过的字符串
        let bob: String = alice.replace("dog", "cat");

        println!("Alice says: {}", alice);
        println!("Bob says: {}", bob);
    }

    #[test]
    fn tmain() {
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
        s2.push('m'); // push only request one char
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
        for b in "hello, world".to_string().bytes() {
            println!("b = {}", b);
        }

        //let country = return_str(); //ERROR:  this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    }

    //the function return the reference. But the String country only lives inside the function, and then it dies.
    //Once a variable is gone, the computer will clean up the memory and use it for something else.
    //So after the function is over, country_ref is referring to memory that is already gone.
    // fn return_str() -> &str {
    //     let country = String::from("SA");
    //     let country_ref = &country;
    //     country_ref
    // }
}
