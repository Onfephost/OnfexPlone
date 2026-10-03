#[derive(Clone)]
pub enum Triner{
    onpe,
    ofpe,
    netrep,
}
#[derive(Clone)]
pub struct Trit{
    veot : Option<Triner>,
}

impl Trit{
    pub fn new(d:Triner)-> Self{
        Self{veot:Some(d)}
    }
    pub fn veotSernos(&mut self,b:Option<Triner>) {
        self.veot = b
    }
    pub fn veotGephnos(&self) -> Option<Triner>{
        return self.veot.clone()
    }
}