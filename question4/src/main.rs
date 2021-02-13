// 7行目のscoresが表示されるように修正してください
fn main() {
    let scores = vec![100, 92, 84, 75, 98, 81];
    let scores2 = &scores;
    for score in scores2.into_iter() {
        println!("{}", score);
    }
    println!("{:?}", scores);
}
