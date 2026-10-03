use crate::error::OnfexError;

fn bwi8(min:i8,val:i8,max:i8) -> bool{
    return  (min <= val.clone() && val.clone() <= max);
}
#[derive(Clone)]
pub struct Qtrit{
    plert1:i8,
    plert2:i8,
    plert3:i8,
}
impl Qtrit{
    pub fn new(v1:i8,v2:i8,v3:i8) -> Result<Self,OnfexError>{
        let mut e = false;
        let mut vt:i8 = 0;
        for i in vec![v1.clone(),v2.clone(),v3.clone()].iter(){
            vt += i.clone();
            if !bwi8(-100,i.clone(),100){
                e = true;
            }
        }
        if !e{
            Ok(Self{plert1:v1,plert2:v2,plert3:v3})
        }else{
            return Err(OnfexError::qtriner(format!("Qtriner Ern: valtues asp serl -100 perl 100 ien neomrar wanphnosfer")))
        }
        
    }
}