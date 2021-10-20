use crate::rand::Rand;

pub struct Pom{
    id:i32,
    list:Vec<Pom>,
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

    pub fn copy(&self)->Pom{
        let mut p = Pom::new(self.id);
        p.list = self.list.iter().map(|x| x.copy()).collect();
        p.point = self.point;
        p
    }

    pub fn add_list(&mut self, p:Pom){
        self.list.push(p);
    }

    pub fn calc_point(&self)->f32{
        let mut rng = Rand::new(self.id as u32);
        self.list.iter().map(|x| x.point * rng.random() as f32).sum()
    }
}
