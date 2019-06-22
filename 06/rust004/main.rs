use std::io;
use std::collections::HashMap;
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

// https://github.com/hatoo/competitive-rust-snippets/blob/5b6bff04e74045766595421b680f7175f209ae3b/src/uft.rs
/// Union Find Tree
pub struct UFT {
    pub par: Vec<usize>,
    pub rank: Vec<usize>,
}

impl UFT {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        UFT {
            par: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    #[allow(dead_code)]
    pub fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            let p = self.par[x];
            let pp = self.root(p);
            self.par[x] = pp;
            pp
        }
    }

    #[allow(dead_code)]
    pub fn merge(&mut self, x: usize, y: usize) {
        let x = self.root(x);
        let y = self.root(y);
        if x == y {
            return;
        }

        if self.rank[x] < self.rank[y] {
            self.par[x] = y;
        } else {
            self.par[y] = x;
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }
}

/// 

fn main(){
    let n = read_int() as usize;
    let xs = {
        let mut temp = vec![];
        for _ in 0..n{
            let ys = read_ints();
            temp.push((ys[0],ys[1]));
        }
        temp
    };

    let mut hm_x : HashMap<i64,usize> = HashMap::new();
    let mut hm_y : HashMap<i64,usize> = HashMap::new();
    let mut uf = UFT::new(n);

    for (i,v) in xs.iter().enumerate(){
        let (x,y) = v.clone();

        if hm_x.contains_key(&x){
            uf.merge(i,*hm_x.get(&x).unwrap());
        }else{
            hm_x.insert(x,i);
        }
        if hm_y.contains_key(&y){
            uf.merge(i,*hm_y.get(&y).unwrap());
        }else{
            hm_y.insert(y,i);
        }
    }

    let mut checked_root= HashMap::new(); 
    let mut result = 0;
    for i in 0..n{
        let root = uf.root(i);
        if checked_root.contains_key(&root){
            continue;
        }else{
            checked_root.insert(root,true);
        }

        let mut tx = HashMap::new();
        let mut ty = HashMap::new();
        let mut cnt = 0;

        //println!("not we count root = {}",root);
        for j in i..n{
            if root != uf.root(j){
                continue;
            }
            //println!("{} is friend!",j);

            let (x,y) = xs[j].clone();
            
            if tx.contains_key(&x) == false{
                tx.insert(x,true);
            }
            if ty.contains_key(&y) == false{
                ty.insert(y,true);
            }
            cnt += 1;
        }


        if cnt >=  3 {
            let nx = tx.len();
            let ny = ty.len();

            if nx * ny != 0{
                //println!("{}",nx);
                //println!("{}",ny);
                //println!("{}",cnt);
                result += nx*ny - cnt;
            }
        }
    }
    
    println!("{}",result);

}






































