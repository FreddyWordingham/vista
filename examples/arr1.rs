use ndarray::arr1;
use vista::{CommaSeparated, DisplayExt, DoubleJoined, Joined, Separated};

fn main() {
    let a = arr1(&[1.0, 2.0, 3.0]);

    println!("Separated");
    println!("{}\n", a.display::<Separated>());

    println!("CommaSeparated");
    println!("{}\n", a.display::<CommaSeparated>());

    println!("Joined");
    println!("{}\n", a.display::<Joined>());

    println!("DoubleJoined");
    println!("{}", a.display::<DoubleJoined>());
}
