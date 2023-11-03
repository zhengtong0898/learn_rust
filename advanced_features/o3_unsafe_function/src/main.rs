use std::slice;

// 使用安全的函数对不安全的代码进行封装.
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    // 通过断言确认数据的有效性, 这是使用unsafe代码的最佳方式.
    assert!(mid <= len);

    // 不安全的代码块
    unsafe {
        (
            // slice::from_raw_parts_mut 是一个不安全的函数, 所以只能在unsafe关键字作用域内使用它.
            // slice::from_raw_parts_mut 不会对参数进行检查, 它100%信任参数是有效的, 如果参数是无效的那么将会
            //                           在运行时panic, 因此最好的办法就是提前给参数做好断言确认参数是有效的.
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

#[allow(unused_variables)]
fn main() {
    let mut vector = vec![1, 2, 3, 4, 5, 6];

    // 由于 split_at_mut 已经将不安全代码封装起来, 因此它是一个安全函数, 可以直接调用.
    let (left, right) = split_at_mut(&mut vector, 3);

    // ss == 0xc57f2f52c == 1;  // 这是栈上的地址格式, 1存储在栈上.
    let ss = 1;
    println!("{:p}", &ss as *const i32);

    // left[0]   == 0x15133227370 == 1;   // 这是堆上的地址格式, 1存储在堆上
    // vector[0] == 0x15133227370 == 1;   // 这是堆上的地址格式, 1存储在堆上
    println!("{:p}", left.get(0).unwrap() as *const i32);
    println!("{:p}", vector.get(0).unwrap() as *const i32);

    // 这是一个无效索引的场景, 可以顺利通过编译, 但是会在运行时报错.
    invalid_index_will_panic();
}

fn invalid_index_will_panic() {
    let address = 0x01234usize;
    let r = address as *mut i32;

    // 对不安全代码的封装
    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

    // 如果不使用、不引用values这个无效地址的变量, 就不会报错, 否则就会报错.
    println!("{:?}", values);
}
