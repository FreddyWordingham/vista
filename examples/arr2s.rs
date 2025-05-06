use ndarray::arr2;
use vista::{CommaSeparated, DisplayExt, DoubleJoined, Joined, Separated};

fn main() {
    let a = arr2(&[[1.0, 2.0], [3.0, 4.0]]);
    let b = arr2(&[[5.0, 6.0], [7.0, 8.0]]);
    let c = arr2(&[[9.0, 10.0], [11.0, 12.0]]);
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
