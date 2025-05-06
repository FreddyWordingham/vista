use ndarray::arr1;
use vista::{CommaSeparated, DisplayExt, DoubleJoined, Joined, Separated};

fn main() {
    let a = arr1(&[1.0, 2.0, 3.0]);
    let b = arr1(&[4.0, 5.0, 6.0]);
    let c = arr1(&[7.0, 8.0, 9.0]);
    let arrs = [&a, &b, &c];

    println!("Separated");
    println!("{}\n", arrs.display::<Separated>());

    println!("CommaSeparated");
    println!("{}\n", arrs.display::<CommaSeparated>());

    println!("Joined");
    println!("{}\n", arrs.display::<Joined>());

    println!("DoubleJoined");
    println!("{}", arrs.display::<DoubleJoined>());
}
