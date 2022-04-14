fn main() {
    let u32_vector = [1,45,65,7];
    println!("求和结果为：{:?}",u32_sum(&u32_vector));
}
fn u32_sum(num_vector: &[u32])-> Option<u32> {
    let mut sum:u32 = 0;
    // 遍历vector
    for num in num_vector {
        // check_*函数返回Option，一旦发生溢出则返回None
        match sum.checked_add(*num) {
            Some(s) => {
                sum = s;
            },
            None => {
                return None;
            },
        };
    }
    Some(sum)
}
