use std::io;

fn read_line() -> String{
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn read_ints() -> Vec<i64>{
    let s = read_line();
    let split:Vec<&str> = s.split(" ").collect();
    split.iter().map(|&x| x.to_string().parse().unwrap()).collect()
}

fn read_int() -> i64{
    read_ints()[0]
}

fn main(){
    println!("max digit>>");
    let max_digit = read_int();
    println!("target value>>");
    let target : f64 = read_line().parse().unwrap();

    let max_val = {
        let mut ret = 1;
        for _i in 0..max_digit{
            ret *= 10;
        }
        ret
    };

    let mut min_error = 1e100;
    let mut min_upper = 1;
    let mut min_lower = 1;

    for u in 0..max_val{
        for l in 0..max_val{
            let v = u as f64 / l as f64;
            let e = (target - v).abs();

            if min_error > e{
                min_error = e;
                min_upper = u;
                min_lower = l;
            }
        }
    }

    println!("{}/{}={}",min_upper,min_lower,min_upper as f64/min_lower as f64);
    
}


