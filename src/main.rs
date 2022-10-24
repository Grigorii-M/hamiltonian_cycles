use ham_cycles::*;

fn main() {
    let labels = vec!["a", "b", "c", "d", "e"];
    const NUM_VERTICES: u32 = 5;

    #[rustfmt::skip]
    let m = symbolic_matrix!(
        NUM_VERTICES, NUM_VERTICES,
        [
            0, b, c, 0, e,
            0, 0, c, 0, e,
            0, b, 0, d, 0,
            0, b, 0, 0, e,
            a, 0, c, 0, 0,
        ]
    );

    #[rustfmt::skip]
    let mut v = symbolic_matrix!(
        NUM_VERTICES, NUM_VERTICES,
        [
            0, 1, 1, 0, 1,
            0, 0, 1, 0, 1,
            0, 1, 0, 1, 0,
            0, 1, 0, 0, 1,
            1, 0, 1, 0, 0,
        ]
    );

    for _ in 0..4 {
        v = m.clone() * v;
    }

    let diag = v.diagonal();
    let mut data = vec![];
    for i in 0..NUM_VERTICES {
        let val = diag[i as usize].clone();
        data.push(val);
    }

    match clean_up_data(&data, &labels) {
        Ok(data) => println!("{:#?}", data),
        Err(msg) => eprintln!("{}", msg),
    }
}
