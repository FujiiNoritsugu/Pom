mod pom;
mod rand;
mod consts;

use crate::pom::Pom;

fn main() {
    // 双方向に腹持ちできる構造をつくる。
    // まずは2件
    let mut pom1 = Pom::new(1);
    let mut pom2 = Pom::new(2);
    pom1.add_list(pom2.copy());
    pom2.add_list(pom1.copy());
    
    // 次は100件
    let mut pom_list:Vec<Pom> = Vec::new();
    for i in 0..100{
        pom_list.push(Pom::new(i));
    }

    // 同じ内容のリストをコピーする
    let sub_list:Vec<Pom> = pom_list.iter().map(|x| x.copy()).collect();

    for pom1 in pom_list.iter_mut(){
        for pom2 in sub_list.iter(){
            // ここのpom2.copy()がダサい
            // pom2をいれようとすると&pomやからダメと言われる
            pom1.add_list(pom2.copy());
        }
    }

    for pom in pom_list.iter(){
        // calc_pointが全部おんなじになるなんでや？
        // ポイントも乱数で作成すると少しは違う値になった
        println!("point:{}",pom.calc_point());
    }
}
