// SwordとHeroのメンバ変数nameをstr型のまま、lifetime識別子を使用してコンパイルが通るように修正してください
struct Sword<'a> {
    name: &'a str,
    damage: i32,
}

struct Hero<'b> {
    name: &'b str,
    hp: i32,
    sword: Option<Sword<'b>>,
}

impl Hero<'_> {
    pub fn attack(&self) {
        println!("{}は攻撃した", self.name);
        if let Some(s) = &self.sword {
            println!("敵に{}ダメージ与えた", s.damage);
        } else {
            println!("敵にダメージを与えられない");
        }
    }
}

fn main() {
    let s: Sword = Sword {
        name: "漆黒の剣",
        damage: 10,
    };
    let h: Hero = Hero {
        name: "ひろあき",
        hp: 100,
        sword: Some(s),
    };

    h.attack();
}
