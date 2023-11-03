extern crate quote;
extern crate syn;

/*
    syn::parse_str 要求输入是一个字符串, 并负责将字符串解析成 syn::Parse trait,
    也就是说那些实现了 syn::Parse trait 的 struct 或 enum , 都是合法的返回对象.

    syn::parse 要求输入是一个 proc_macro::TokenStream, 并负责将字符串解析成 syn::Parse trait,
    也就是说那些实现了 syn::Parse trait 的 struct 或 enum , 都是合法的返回对象.

    syn::parse 要求输入是一个 proc_macro2::TokenStream, 并负责将字符串解析成 syn::Parse trait,
    也就是说那些实现了 syn::Parse trait 的 struct 或 enum , 都是合法的返回对象.
*/

fn main() {
    // 使用 syn::parse_str 返回的对象是一种语法树结构.
    let source: &str = "2 + 3 * 4 - 5";
    let parsed_expr: syn::Expr = syn::parse_str(source).expect("Failed to parse expression");

    // 这里内部使用了 syn::visit 来对语法树结构进行递归遍历(visitor pattern 访问者模式), 最终得出数字统计结果.
    let int_literal_count = count_int_literals(&parsed_expr);

    // 打印结果.
    println!(
        "The expression contains {} integer literals.",
        int_literal_count
    );
}

fn count_int_literals(expr: &syn::Expr) -> usize {
    use syn::visit::Visit;
    use syn::ExprLit;

    struct IntLiteralCounter {
        count: usize,
    }

    impl<'ast> Visit<'ast> for IntLiteralCounter {
        fn visit_expr_lit(&mut self, node: &'ast ExprLit) {
            // 如果 node.lit 是一个 int 那么就计数递增1.
            if let syn::Lit::Int(_) = &node.lit {
                self.count += 1;
            }
        }
    }

    let mut counter = IntLiteralCounter { count: 0 };

    // syn::visit::visit_expr 会执行 counter.visit_expr_lit 接口方法, 完成计数.
    syn::visit::visit_expr(&mut counter, expr);

    // 返回计数结果
    counter.count
}
