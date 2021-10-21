use crate::rand::Rand;
use std::rc::Rc;

pub struct Pom{
    id:i32,
    list:Vec<Rc<Pom>>,
    point:f32
}

impl Pom{
    pub fn new(i:i32)->Pom{
        // pointをランダムにしてみる
        let mut rng = Rand::new(i as u32);
        Pom{
            id:i,
            list:Vec::new(),
            point:rng.rand() as f32
        }
    }

    pub fn add_list(&mut self, p:Rc<Pom>){
        self.list.push(Rc::clone(&p));
    }

    pub fn calc_point(&self)->f32{
        let mut rng = Rand::new(self.id as u32);
        self.list.iter().map(|x| x.point * rng.random() as f32).sum()
    }
}
