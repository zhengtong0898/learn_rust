static mut ROUTES: Vec<(&'static str, &'static str, fn())> = Vec::new();

// 这是一个注册路由表的函数, 通常是为宏提供服务的接口函数.
pub fn register_route(method: &'static str, path: &'static str, func: fn()) {
    unsafe {
        ROUTES.push((method, path, func));
    }
}

// 这是一个对外公开的API, 它可以根据参数, 从路由表中找到对应的函数, 并执行函数.
pub fn dispatch_request(method: &str, path: &str) {
    unsafe {
        for (route_method, route, func) in &ROUTES {
            if route == &path && &method == route_method {
                func();
                return;
            }
        }
        println!("404 Not Found: {} {}", method, path);
    }
}
