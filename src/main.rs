use ham_cycles::*;

fn main() {
    #[rustfmt::skip]
    let m = symbolic_matrix!(
        5, 5,
        [
            0, 0, c, d, 0,
            0, 0, 0, d, 0,
            0, b, 0, 0, 0,
            0, 0, c, 0, e,
            a, b, 0, 0, 0,
        ]
    );

    let mut v = symbolic_vector!(0, 0, 1, 0, 1);

    for _ in 0..5 {
        v = m.clone() * v;
        println!("{:#?}", v);
    }
}
