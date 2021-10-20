mod pom;
use crate::pom::Pom;

fn main() {
    // 双方向に腹持ちできる構造をつくる。
    // まずは100件
    let mut pom1 = Pom::new(1);
    let mut pom2 = Pom::new(2);
    pom1.add_list(pom2.copy());
    pom2.add_list(pom1.copy());
}

