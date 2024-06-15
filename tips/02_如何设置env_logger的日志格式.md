### 如何设置env_logger的日志格式
1. 设置时间格式: "%Y-%m-%d %H:%M:%S.%6f"
2. 设置路径位置: get_path_startswith_project
```
use log::info;
use std::borrow::Cow;
use std::env;
use std::io::Write;
use std::path::{Path, PathBuf};

const PROJECT_NAME: &'static str = env!("CARGO_PKG_NAME");

fn get_path_startswith_project<'a>(file_path_str: &str) -> Cow<str> {
    let word = "src";
    let file_path = Path::new(file_path_str);
    let components: Vec<_> = file_path.components().collect();

    // 等同于 startswith("src")
    if file_path.exists() {
        if components.len() > 0 && components[0].as_os_str() == word {
            let project_path = Path::new(PROJECT_NAME);
            let result = project_path.join(file_path).to_string_lossy().into_owned();
            return Cow::Owned(result);
        }
    }

    // 当 word 统计次数大于限定次数, 直接返回原始数据
    if components.iter().filter(|x| x.as_os_str() == word).count() > 2 {
        return Cow::Borrowed(file_path_str);
    }

    // 将路径组件转换为 Vec 并反转
    let mut reversed_components = components.iter().rev();

    // 找到 "src" 的上一级目录的位置
    if let Some(src_index) = reversed_components.position(|&part| part.as_os_str() == word) {
        let start_index = components.len() - src_index - 2;
        let project_subpath: PathBuf = components[start_index..].iter().collect();
        let result = project_subpath.to_string_lossy().to_string();
        return Cow::Owned(result);
    }

    Cow::Borrowed(file_path_str)
}

pub fn log_init() {
    let env = env_logger::Env::default().default_filter_or("debug");             // 设置日志级别
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}:L{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S.%6f"),
                record.level(),
                get_path_startswith_project(record.file().unwrap_or("unknown")), // 切割路径: 从项目名开始到具体的文件名
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
}

fn main() {
    log_init();

    info!("hello world!");
}
```

输出结果
```
[2024-06-15 11:59:24.986893 INFO untitled\src\main.rs:61] hello world!
```