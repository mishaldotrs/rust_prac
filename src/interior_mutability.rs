use std::cell::RefCell;

#[allow(dead_code)]
pub fn run(){
    let data = RefCell::new(5);

    {
        let mut mut_ref = data.borrow_mut();
        *mut_ref += 10;
    }

    println!("{}",data.borrow());
}
