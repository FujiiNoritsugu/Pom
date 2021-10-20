pub struct Pom<'a>{
    id:i32,
    list:Vec<&'a Pom<'a>>,
    point:f32
}

impl Pom<'_>{
    pub fn new(i:i32)->Pom<'static>{
        Pom{
            id:i,
            list:Vec::new(),
            point:0.0
        }
    }
    pub fn add_list(&mut self, p:&Pom){
        self.list.push(p);
    }
}
