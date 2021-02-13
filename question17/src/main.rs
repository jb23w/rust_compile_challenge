// mainのコードが実行できるように構造体を修正してください
struct Hoge<T, U> {
    a: T,
    b: U,
}
impl<T,U> Hoge<T,U> {
    fn valueA(&self)-> &T{
        &self.a
    }
    fn valueB(&self)-> &U{
        &self.b
    }
}
fn main() {
    let x = Hoge { a: 1, b: "aiueo" };

    println!("{}, {}", x.valueA(), x.valueB());
}
