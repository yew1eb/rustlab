use std::collections::HashSet;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tmain() {
        let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
        let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

        assert!(a.insert(4));
        assert!(a.contains(&4));

        // 如果值已经存在，那么 `HashSet::insert()` 返回 false。
        // assert!(b.insert(4), "Value 4 is already in set B!");
        // 改正 ^ 将此行注释掉。

        b.insert(5);

        // 若一个组合的元素类型实现了 `Debug`，那么该组合也就实现了 `Debug`。
        // 这通常将元素打印成这样的格式 `[dlem1, elem2, ...]
        println!("A: {:?}", a);
        println!("B: {:?}", b);

        // 乱序打印 [1, 2, 3, 4, 5]。
        println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());

        // 这将会打印出 [1]
        println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());

        // 乱序打印 [2, 3, 4]。
        println!(
            "Intersection: {:?}",
            a.intersection(&b).collect::<Vec<&i32>>()
        );

        // 打印 [1, 5]
        println!(
            "Symmetric Difference: {:?}",
            a.symmetric_difference(&b).collect::<Vec<&i32>>()
        );
    }

    #[test]
    fn t_hashset() {
        let many_numbers = vec![
            94, 42, 59, 64, 32, 22, 38, 5, 59, 49, 15, 89, 74, 29, 14, 68, 82, 80, 56, 41, 36, 81,
            66, 51, 58, 34, 59, 44, 19, 93, 28, 33, 18, 46, 61, 76, 14, 87, 84, 73, 71, 29, 94, 10,
            35, 20, 35, 80, 8, 43, 79, 25, 60, 26, 11, 37, 94, 32, 90, 51, 11, 28, 76, 16, 63, 95,
            13, 60, 59, 96, 95, 55, 92, 28, 3, 17, 91, 36, 20, 24, 0, 86, 82, 58, 93, 68, 54, 80,
            56, 22, 67, 82, 58, 64, 80, 16, 61, 57, 14, 11,
        ];

        let mut number_hashset = HashSet::new();

        for number in many_numbers {
            number_hashset.insert(number);
        }

        let hashset_length = number_hashset.len(); // The length tells us how many numbers are in it
        println!(
            "There are {} unique numbers, so we are missing {}.",
            hashset_length,
            100 - hashset_length
        );

        // Let's see what numbers we are missing
        let mut missing_vec = vec![];
        for number in 0..100 {
            if number_hashset.get(&number).is_none() {
                // If .get() returns None,
                missing_vec.push(number);
            }
        }

        print!("It does not contain: ");
        for number in missing_vec {
            print!("{} ", number);
        }
    }
}
