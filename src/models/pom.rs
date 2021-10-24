use rand::Rng;
use std::rc::Rc;
use std::cell::RefCell;
use crate::enums::qom::Qom;

#[derive(Debug)]
pub struct Pom{
    id:i32,
    list:Vec<Rc<RefCell<Pom>>>,
    pub point:f32,
    pub abbrev:Qom,
}

impl Pom{
    pub fn new(i:i32)->Pom{
        // pointをランダムにしてみる
        let mut rng = rand::thread_rng();
        Pom{
            id:i,
            list:Vec::new(),
            point:rng.gen(),
            abbrev:Qom::None,
        }
    }

    pub fn add_list(&mut self, p:Rc<RefCell<Pom>>){
        self.list.push(p);
    }

    pub fn calc_point(&self)->f32{
        // 腹持ちのデータからランダムな値の積和を計算するようにする
        let mut rng = rand::thread_rng();
        self.list.iter().map(|x| x.borrow().point * rng.gen::<f32>()).sum()
    }

    pub fn set_abbrev(&mut self, ave:f32){
        // 自分のポイントが平均よりも上ならばRichを下ならばPoorを設定する
        if self.point > ave{
            self.abbrev = Qom::Rich
        }else{
            self.abbrev = Qom::Poor
        }
    }
}
