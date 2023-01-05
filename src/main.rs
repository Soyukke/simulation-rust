trait ISolver {
    fn hello(&self);
}

struct Solver {
    count: i32
}

impl ISolver for Solver {
   fn hello(&self) {
      println!("hello: {}", self.count); 
   } 
}

fn main() {
    println!("Hello, world!");
    let hoge = Solver{
        count: 10
    };
    hoge.hello();
}
