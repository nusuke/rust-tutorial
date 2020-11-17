fn main() {
    println!("Hello, world!");
    println!("{}",Module::Module2::isOdd(10));
    println!("{}",ModuleA::isEven(10));
}

mod Module {
    pub mod Module2{
        pub fn isOdd(num: i32) -> bool{
            return num %2 == 0;

        }
    }
}

mod ModuleA {
    pub fn isEven(num :i32) -> bool{
        return !crate::Module::Module2::isOdd(num)
    }
}