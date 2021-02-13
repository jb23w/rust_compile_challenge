// hello worldが出力されるように修正してください
fn helloworld<'a>() -> &'a str {
    "hello world"
}

fn main() {
    println!("{}", helloworld());
}
