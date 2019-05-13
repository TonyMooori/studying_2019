struct A;
struct B;

trait SampleTrait{
    fn test(&self);
}

impl SampleTrait for A{
    fn test(&self){
        println!("This is A");
    }
}

impl SampleTrait for B{
    fn test(&self){
        println!("This is B");
    }
}


fn main(){
   let a = A{};
   let b = B{};

   let vs:Vec<Box<SampleTrait>> = vec![
       Box::new(a),
       Box::new(b),
   ];
    
   for v in vs{
       v.test();
   }
}
