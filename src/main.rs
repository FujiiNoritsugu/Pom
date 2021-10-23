mod models;
mod enums;

use crate::models::pom::Pom;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // 双方向に腹持ちできる構造をつくる。
    // 20211022 コピーをRcを使用して書き直してみる
    // 100件分のリストを作成
    let mut pom_list:Vec<Rc<RefCell<Pom>>> = Vec::new();
    for i in 0..100{
        pom_list.push(Rc::new(RefCell::new(Pom::new(i))));
    }

    // 同じ内容のリストをコピーする
    let sub_list:Vec<Rc<RefCell<Pom>>> = pom_list.iter().map(|x| Rc::clone(&x)).collect();

    for pom1 in pom_list.iter_mut(){
        for pom2 in sub_list.iter(){
            // ここのpom2.copy()がダサい→20211022 ここをRcを使用して変更してみる
            // pom2をいれようとすると&pomやからダメと言われる
            // Rcのクローンでいれようとすると「cannot borrow data in an `Rc` as mutable」と言われる
            // RcとRefCellの組み合わせでコピーなしでできた。
            pom1.borrow_mut().add_list(Rc::clone(pom2));
        }
    }

    for pom in pom_list.iter(){
        // calc_pointが全部おんなじになるなんでや？
        // →ポイントも乱数で作成すると少しは違う値になった
        println!("point:{}",pom.borrow().calc_point());
        // ポイントの平均値をとって上ならRichを下ならPoorに設定する
        println!("qom:{:?}", pom.borrow());
    }

    // ポイントの平均値をとる
    
}
