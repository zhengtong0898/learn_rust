fn main() {
    // 创建一个常规变量
    let mut num = 5;

    // 1. raw pointer
    //    `raw pointer` 的形式是: `*const T` 和 `*mut T`
    //    从一个`r1、r2`常规引用中使用 `as` 来创建 `raw pointer`, 意味着这是一个有效的指针.
    let r1 = &num as *const i32; // 创建一个不可变的raw pointer.
    let r2 = &mut num as *mut i32; // 创建一个可变的raw pointer.

    // 2. 随便创建一个"地址"变量, 并将它转换成 `raw pointer`, 这意味着不是一个有效指针, 但是并没有报错.
    let address: usize = 0x012345;
    let r3 = address as *const i32;

    // 3. 如果不使用 `unsafe` 关键字开辟作用域、函数、方法, 那么就无法对r1、r2进行解引用.
    // println!("r1 is: {}", *r1); // 这里会报错, 因为没有使用unsafe关键字.

    // 4. 使用 `unsafe` 关键字开辟作用域, 对r1、r2进行解引用.
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        println!("r3 is: {:?}", r3);
        // println!("r3 is: {}", *r3); // 这里会报错, 因为访问到无效指针.
    }
}
