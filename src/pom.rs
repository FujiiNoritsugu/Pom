use crate::rand::Rand;
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
        let mut rng = Rand::new(i as u32);
        Pom{
            id:i,
            list:Vec::new(),
            point:rng.random() as f32
        }
    }

    pub fn add_list(&mut self, p:Rc<RefCell<Pom>>){
        self.list.push(p);
    }

    pub fn calc_point(&self)->f32{
        let mut rng = Rand::new(self.id as u32);
        self.list.iter().map(|x| x.borrow().point * rng.random() as f32).sum()
    }
}
