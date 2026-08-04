trait Greet {
    fn say_hello(&self){
        println!("hello");
    }
}


struct GoodMorning;

impl Greet for GoodMorning{
    fn say_hello(&self) {
        println!("good morning ji");
    }
}


fn main(){
   let goodmorning = GoodMorning;
   goodmorning.say_hello();
}