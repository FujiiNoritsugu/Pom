use rand::Rng;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Pom{
    id:i32,
    list:Vec<Rc<RefCell<Pom>>>,
    point:f32
}

impl Pom{
    pub fn new(i:i32)->Pom{
        // pointをランダムにしてみる
        let mut rng = rand::thread_rng();
        Pom{
            id:i,
            list:Vec::new(),
            point:rng.gen()
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
}
