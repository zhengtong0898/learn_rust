extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn measure_time(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // 将 item 解析为一个函数.
    let input_fn = parse_macro_input!(item as ItemFn);

    // 获取函数名
    let fn_name = &input_fn.sig.ident;
    // 获取函数体的代码
    let fn_block = &input_fn.block;
    // 获取函数签名, 包含: 函数名 + 形参 + 返回值类型
    let fn_sig = &input_fn.sig;

    // 装饰器风格的代码
    let output = quote! {
        #fn_sig {
            let start_time = std::time::Instant::now();
            // 这里相当于是开辟一个作用域去执行代码
            // 例如:
            // fn do_something() -> bool {
            //     false
            // }
            // 这里会变成:
            // let result: bool = {
            //     false
            // }
            // 它会即时的去执行这段代码.
            let result: bool = #fn_block;
            let elapsed_time = start_time.elapsed();
            println!("fn_name: {}, fn_sig: {}, fn_block: {}, executed in {:?}", stringify!(#fn_name), stringify!(#fn_sig), stringify!(#fn_block), elapsed_time);
            result
        }
    };

    // 上面那段代码把result的类型写死了, 限制了宏的灵活性,
    // 下面这段代码是根据函数的返回类型 ret_type 来进行模式匹配生成符合条件的代码.
    // let ret_type = &input_fn.sig.output;
    // let output = match ret_type {
    //     syn::ReturnType::Default => {
    //         quote! {
    //             #fn_sig {
    //                 let start_time = std::time::Instant::now();
    //                 #fn_block
    //                 let elapsed_time = start_time.elapsed();
    //                 println!("Function {}, function_signature {},  executed in {:?}", stringify!(#fn_name), stringify!(#fn_sig), elapsed_time);
    //             }
    //         }
    //     }
    //     syn::ReturnType::Type(_, ret_ty) => {
    //         quote! {
    //             #fn_sig {
    //                 let start_time = std::time::Instant::now();
    //                 let result: #ret_ty = #fn_block;
    //                 let elapsed_time = start_time.elapsed();
    //                 println!("Function {}, function_signature {}, executed in {:?}", stringify!(#fn_name), stringify!(#fn_sig), elapsed_time);
    //                 result
    //             }
    //         }
    //     }
    // };

    output.into()
}
