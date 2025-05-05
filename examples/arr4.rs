use ndarray::Array4;
use vista::{CommaSeparated, DisplayExt, DoubleJoined, Joined, Separated};

fn main() {
    let a = Array4::from_shape_vec(
        (2, 2, 2, 2),
        vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        ],
    )
    .unwrap();

    println!("Separated");
    println!("{}\n", a.display::<Separated>());

    println!("CommaSeparated");
    println!("{}\n", a.display::<CommaSeparated>());

    println!("Joined");
    println!("{}\n", a.display::<Joined>());

    println!("DoubleJoined");
    println!("{}", a.display::<DoubleJoined>());
}
