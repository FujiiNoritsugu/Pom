mod pom;
mod rand;
mod consts;
mod enums;

use std::rc::Rc;
use std::cell::RefCell;
use crate::pom::Pom;

fn main() {
    // 双方向に腹持ちできる構造をつくる。
    // まずは2件
    let mut pom1 = Pom::new(1);
    let mut pom2 = Pom::new(2);
    pom1.add_list(pom2.copy());
    pom2.add_list(pom1.copy());
    
    // 次は100件
    let pom_list_rc:Rc<RefCell<Vec<Pom>>> = Rc::new(RefCell::new(Vec::new()));
    for i in 0..100{
        pom_list_rc.borrow_mut().push(Pom::new(i));
    }

    // 同じ内容のリストをコピーする
    // let sub_list:Vec<Pom> = pom_list.iter().map(|x| x.copy()).collect();
    let outer_list = Rc::clone(&pom_list_rc);
    for pom1 in outer_list.borrow_mut().iter_mut(){
        let inner_list = Rc::clone(&pom_list_rc);
        for pom2 in inner_list.borrow_mut().iter(){
            // ここのpom2.copy()がダサい
            // pom2をいれようとすると&pomやからダメと言われる
            pom1.add_list(pom2.copy());
        }
    }

    for pom in pom_list_rc.borrow().iter(){
        // calc_pointが全部おんなじになるなんでや？
        // ポイントも乱数で作成すると少しは違う値になった
        println!("point:{}",pom.calc_point());
    }
}
