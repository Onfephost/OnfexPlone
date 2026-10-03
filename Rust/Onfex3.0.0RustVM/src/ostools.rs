use crate::error::OnfexError;
use std::collections::HashMap;
use std::process::Command;

pub fn command(c:String,args:Vec<String>) -> String{
    let output = Command::new(c.clone())
        .args(args.clone()).output().unwrap();
    
    let text = String::from_utf8(output.stdout).unwrap();
    let chars: Vec<char> = text.chars().collect();
    
    let l = text.chars().count()-1;
    let res:String = chars[0..l].iter().collect();
    format!("{}",res)
}

pub fn autocommand(a:String) -> String{
    let mut v:Vec<String> = a.split(" ").map(|x| x.to_string()).collect();
    let cmd = v[0].clone();
    v.remove(0);
    let args = v.clone();
    return command(cmd,args);
}
