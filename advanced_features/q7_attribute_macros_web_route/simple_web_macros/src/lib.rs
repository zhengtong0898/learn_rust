extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, ItemFn, LitStr, Result};

// 当前宏代码的业务场景
// 1. 找到所有声明 #[route()] 的函数
// 2. 解析宏参数
// 3. 生成一段代码, 这段代码是采用宏解析参数组合而成的函数,
//    用于将函数注册到路由表中, 使 '方法', '路径', '函数' 形成绑定关系.

struct RouteArgs {
    method: LitStr,
    path: LitStr,
}

impl Parse for RouteArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        // 执行一次, 表示尝试从流中解析一个值
        let method: LitStr = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        // 再执行一次, 表示尝试从流中再解析一个值
        let path: LitStr = input.parse()?;
        Ok(RouteArgs { method, path })
    }
}

// 过程宏中的: 属性宏
#[proc_macro_attribute]
// 第一个参数 attr: 是参数, 比如说: #[route("GET", "/Dashboard")] 中的, '"GET", "/Dashboard"' 就是一个TokenStream.
// 第二个参数 item: 是函数体, 即: 被修饰的函数对象.
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 在宏函数内, as与常规代码中的不一样, 在这里 as是类型注解.
    // parse_macro_input! 宏专门用来解析TokenStream, 支持解析到自定义结构.
    // TokenStream.parse() 每执行一次, 会尝试从流中解析一个值.
    let args = parse_macro_input!(attr as RouteArgs); // let args: RouteArgs = parse_macro_input!(attr);      // 等价的代码
    let input = parse_macro_input!(item as ItemFn); // let input: ItemFn = parse_macro_input!(item);             // 等价的代码
    let func_name = &input.sig.ident;

    let method = args.method.value().to_uppercase();
    let path = args.path.value();

    let register_name = format!("register_{}", func_name);
    let register_name_ident = syn::Ident::new(&register_name, func_name.span());

    // 采用ctor在运行时自动执行#register_name_ident函数, 完成路由表注册动作.
    let expanded = quote! {
        #[ctor::ctor]
        fn #register_name_ident() {
            simple_web_framework::register_route(#method, #path, #func_name);
        }

        #input
    };

    expanded.into()
}
