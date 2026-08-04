trait Speak {
    // default method implementation
    fn speak(&self){
        pritln!("janwar apne hisab se bhokenga");
    }
}

struct Admi;

impl Speak for Admi{}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

impl Speak for Dog {
    fn speak(&self) {
        println!("{} speaks: woof!", self.name);
    }
}

impl Speak for Cat {
    fn speak(&self) {
        println!("{} speaks: meow!", self.name);
    }
}

// ========== Trait Bound wala generic function ==========
fn make_speak<T: Speak>(animal: T) {
    animal.speak();
}

fn main() {
    let kutta = Dog {
        name: String::from("Badal"),
    };

    let billi = Cat {
        name: String::from("pussy"),
    };

    // Ab same function dono types pe kaam karega
    make_speak(kutta);
    make_speak(billi);


    let admi_kyabolta_hain = Admi;
    admi.speak();
}