use simple_web_framework::dispatch_request;
use simple_web_macros::route;

// 采用宏封装, 让用户可以像flask那样定义路由
#[route("GET", "/")]
fn home() {
    println!("Home page accessed via GET!");
}

// 采用宏封装, 让用户可以像flask那样定义路由
#[route("POST", "/about")]
fn about() {
    println!("About page accessed via POST!");
}

fn main() {
    // 模拟浏览器访问API
    dispatch_request("GET", "/");
    dispatch_request("POST", "/about");
    dispatch_request("GET", "/about");
}
