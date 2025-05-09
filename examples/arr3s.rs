use ndarray::arr3;
use vista::{CommaSeparated, DisplayExt, DoubleJoined, Joined, Separated};

fn main() {
    let a = arr3(&[
        [[1.0, 2.0], [3.0, 4.0]],
        [[5.0, 6.0], [7.0, 8.0]],
        [[9.0, 10.0], [11.0, 12.0]],
    ]);
    let b = arr3(&[
        [[13.0, 14.0], [15.0, 16.0]],
        [[17.0, 18.0], [19.0, 20.0]],
        [[21.0, 22.0], [23.0, 24.0]],
    ]);
    let c = arr3(&[
        [[25.0, 26.0], [27.0, 28.0]],
        [[29.0, 30.0], [31.0, 32.0]],
        [[33.0, 34.0], [35.0, 36.0]],
    ]);
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
