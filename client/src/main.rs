//this crate contains only requests to the server crate

//testing lambda pssing to enum

fn do_stuff(func: Box<dyn Fn(&mut i32)>, x: &mut i32) {
    func(x);
}

fn main() {
    let func = |x: &mut i32| *x += 1;
    let mut x = 0;
    do_stuff(Box::new(func), &mut x);
    println!("{}", x); // Output: 1
}
