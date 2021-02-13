// personが出力できるように構造体だけ修正してください
#[derive(Debug)]
struct Person<T> {
    name: T,
}

fn main() {
    let person = Person { name: "zakku" };
    println!("{:?}", person);
}
