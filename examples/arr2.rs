use ndarray::arr2;
use vista::{CommaSeparated, DisplayExt, DoubleJoined, Joined, Separated};

fn main() {
    let a = arr2(&[[1.0, 2.0], [3.0, 4.0]]);

    println!("Separated");
    println!("{}\n", a.display::<Separated>());

    println!("CommaSeparated");
    println!("{}\n", a.display::<CommaSeparated>());

    println!("Joined");
    println!("{}\n", a.display::<Joined>());

    println!("DoubleJoined");
    println!("{}", a.display::<DoubleJoined>());
}
