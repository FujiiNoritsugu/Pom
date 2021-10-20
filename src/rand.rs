use crate::consts::RAND_MAX;

pub struct Rand{
    seed: u32
}

impl Rand{
    // 生成
    pub fn new(x: u32)->Rand{
        Rand{seed:x}
    }   

    // 整数　(0 - RAND_MAX)の一様乱数
    pub fn rand(&mut self) -> u32{
        let x = self.seed as u64;
        self.seed = ((69069 * x + 1) & RAND_MAX as u64) as u32;
        self.seed
    }

    // 実数（0.0以上 1.0未満）の一様乱数
    pub fn random(&mut self) -> f64{
        (1.0 / (RAND_MAX as f64 + 1.0)) * self.rand() as f64
    }

    // 配列をシャッフルする
    pub fn shuffle<T>(&mut self,  buff: &mut [T]){
        for i in 0 .. buff.len(){
            let j = (self.random() * buff.len() as f64) as usize;
            buff.swap(i, j);
        }
    }
}