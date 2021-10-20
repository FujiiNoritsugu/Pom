pub struct Pom{
    id:i32,
    list:Vec<Pom>,
    point:f32
}

impl Pom{
    pub fn new(i:i32)->Pom{
        Pom{
            id:i,
            list:Vec::new(),
            point:0.0
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
}
