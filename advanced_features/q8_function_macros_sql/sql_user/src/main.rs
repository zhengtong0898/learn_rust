use sql_macro::sql;

fn main() {
    let statement = sql!("SELECT * FROM users");
    println!("{}", statement);
}
