use std::thread;

static NTHREADS: i32 = 10;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // 这是主（`main`）线程
    fn tmain() {
        // 提供一个 vector 来存放所创建的子线程（children）。
        let mut childern = vec![];

        for i in 0..NTHREADS {
            // 启动（spin up）另一个线程
            childern.push(thread::spawn(move || {
                println!("this is thread number {}", i)
            }));
        }

        for child in childern {
            // 等待线程到结束。返回一个结果。
            let _ = child.join();
        }
    }
}
