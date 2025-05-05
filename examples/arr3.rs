use ndarray::arr3;
use vista::{CommaSeparated, DisplayExt, DoubleJoined, Joined, Separated};

fn main() {
    let a = arr3(&[
        [[1.0, 2.0], [3.0, 4.0]],
        [[5.0, 6.0], [7.0, 8.0]],
        [[9.0, 10.0], [11.0, 12.0]],
    ]);

    println!("Separated");
    println!("{}\n", a.display::<Separated>());

    println!("CommaSeparated");
    println!("{}\n", a.display::<CommaSeparated>());

    println!("Joined");
    println!("{}\n", a.display::<Joined>());

    println!("DoubleJoined");
    println!("{}", a.display::<DoubleJoined>());
}
