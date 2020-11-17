mod module_a;
mod list_calc;
// use crate::module_a;
// use module_a;

fn main() {

    let vec = vec![3,5,11,11,11,5,5,5,5]; // 1,2,6,4,5

    // println!("Hello, world!");
    // println!("{}",Module::Module2::isOdd(10));
    // println!("{}",module_a::isEven(10));
    println!("{}",list_calc::get_mean(&vec));
    println!("{}",list_calc::get_median(&vec));
    println!("{}",list_calc::get_mode(&vec));
}

mod Module {
    pub mod Module2{
        pub fn isOdd(num: i32) -> bool{
            return num %2 == 0;
        }
    }
}