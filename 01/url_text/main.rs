use std::env;
use std::io::prelude::*;
use std::fs::{OpenOptions,File};

fn main(){
    let args : Vec<String> = env::args().skip(1).collect();
    let mut urls = vec![];

    for arg in args{
        if let Some(url) = get_url(&arg){
            let name = get_name(arg);
            urls.push((name,url));
        }
    }

    let mut file = OpenOptions::new()
        .append(true)
        .open("./urls.md").unwrap();

    for (name,url) in urls{
        let data = format!("* [{}]({})\n",name,url);
        let _ = file.write_all(data.as_bytes());
    }
}

fn get_name(arg:String)->String{
    let pos = match arg.find(".url"){
        Some(p) => p,
        None => return arg
    };

    arg.as_str()[0..pos].to_string()
}

fn get_url(fpath:&String)->Option<String>{
    let mut buf = String::new();
    let mut f = match File::open(fpath.clone()){
        Ok(v) =>v,
        Err(_)=>return None
    };

    if f.read_to_string(&mut buf).is_err(){
        return None;
    }
    
    let lines : Vec<&str> = buf.split('\n').collect();

    for line in lines{
        let line = line.to_string();
        let xs : Vec<&str> = line.trim().split('=').collect();

        if xs.len() == 2 && xs[0] == "URL"{
            return Some(xs[1].to_string());
        }
    }

    None
}

