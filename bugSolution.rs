fn main() {
    let mut x = 5;
    { //Limit the scope of the mutable borrow
        let y = &mut x;
        *y += 1;
    }
    { //Limit the scope of the mutable borrow
        let z = &mut x;
        *z += 1;
    }
    println!("x = {}", x);
}
//Alternative solution using cloning:
//fn main() {
//    let mut x = 5;
//    let mut y = x.clone();
//    let mut z = x.clone();
//    y += 1;
//    z += 1;
//    x = y + z -10; //adjust this line depending on the intended logic
//    println!("x = {}", x);
//}