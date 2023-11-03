use quote::quote;

/*
    quote!宏, 提供了一个类似于模板的语法, 你可以插入变量、重复代码片段, 并生成Rust代码.
*/

fn main() {
    let name = "world";
    // 创建一个Ident标识符结构体
    let name_ident = syn::Ident::new(name, proc_macro2::Span::call_site());

    // 使用 #name_ident 标识符作为变量名, quote! 模板选然后会得到:
    // let world = "world"
    // TODO: 有空需要分析一下quote!的源代码.
    let generated = quote! {
        let #name_ident = #name;
    };

    // 输出生成的代码
    println!("{}", generated.to_string());
}
