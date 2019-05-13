use std::io;
//use std::collections::HashMap;
//use std::collections::BinaryHeap;
//use std::collections::HashSet;
//use std::mem; // mem::swap(&mut x,&mut y);
use std::cmp; // cmp::max,cmp::min

// Rust memo: https://hackmd.io/i0lQY1OKTDW1t66TECOmiw?both

#[allow(dead_code)]
fn read_line() -> String{
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

#[allow(dead_code)]
fn read_charvec() -> Vec<char>{
    read_line().chars().collect()
}

#[allow(dead_code)]
fn read_ints() -> Vec<i64>{
    let s = read_line();
    let split:Vec<&str> = s.split(" ").collect();
    split.iter().map(|&x| x.to_string().parse().unwrap()).collect()
}

#[allow(dead_code)]
fn read_int() -> i64{
    read_ints()[0]
}

#[allow(dead_code)]
fn reverse(s: &String) -> String{
    s.chars().rev().collect::<String>()
}

#[allow(dead_code)]
fn read_lines(n:i64) -> Vec<String>{
    let mut xs = Vec::new();

    for _i in 0..n{
        xs.push(read_line());
    }

    xs
}

#[allow(dead_code)]
fn read_ints2(n:i64) -> Vec<i64>{
    read_lines(n).iter().map(|x| x.parse().unwrap()).collect()
}

#[allow(dead_code)]
fn arange(start:i64,end:i64,step:i64) -> Vec<i64>{
    let mut ret = Vec::new();
    let mut i = start;

    while i < end{
        ret.push(i);
        i += step;
    }

    ret
}

#[allow(dead_code)]
const DXS : [i64; 4] = [0,0,1,-1];
#[allow(dead_code)]
const DYS : [i64; 4]  = [1,-1,0,0];
#[allow(dead_code)]
const INF : i64 = 999999999;
#[allow(dead_code)]
const N_PRIME : i64 = 1000000007;

////////////////////////////////////////////////

#[derive(Clone,Debug)]
enum Color{
    A,
    B,
    C
}

#[derive(Clone,Debug)]
struct Data{
    c:Color,
    val:i64
}

fn main(){	
    let inputs = read_ints();
    let (x,y,z,k) = (inputs[0],inputs[1],inputs[2],inputs[3]);//.4];
    let mut datas = {
        let list_a = read_ints();
        let list_b = read_ints();
        let list_c = read_ints();

        let mut temp = vec![];
        for a in list_a{
            temp.push(Data{c:Color::A,val:a});
        }
        for a in list_b{
            temp.push(Data{c:Color::B,val:a});
        }
        for a in list_c{
            temp.push(Data{c:Color::C,val:a});
        }

        temp
    };


    datas.sort_by_key(|x| -x.val);


    let mut list_a = vec![];
    let mut list_b = vec![];
    let mut list_c = vec![];

    for d in datas{
        match d.c{
            Color::A=>
                list_a.push(d.val),
            Color::B =>
                list_b.push(d.val),
            Color::C =>
                list_c.push(d.val),
        }

        if list_a.len()*list_b.len()*list_c.len() > 100_0000{
            break
        }
    }

    let mut vs = vec![];

    for a in list_a.iter(){
        for b in list_b.iter(){
            for c in list_c.iter(){
                vs.push(a+b+c);
            }
        }
    }

    vs.sort();
    vs.reverse();

    for v in vs.iter().take(k as usize){
        println!("{}",v);
    }
}













