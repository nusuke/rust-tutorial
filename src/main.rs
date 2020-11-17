mod module_a;
// use crate::module_a;
// use module_a;

fn main() {
    println!("Hello, world!");
    println!("{}",Module::Module2::isOdd(10));
    println!("{}",module_a::isEven(10));
}

mod Module {
    pub mod Module2{
        pub fn isOdd(num: i32) -> bool{
            return num %2 == 0;

        }
    }
}