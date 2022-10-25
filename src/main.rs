use ham_cycles::*;

fn main() {
    // let labels = vec!["v1", "v2", "v3", "v4", "v5"];
    // const NUM_VERTICES: u32 = 5;
    //
    // #[rustfmt::skip]
    // let m = symbolic_matrix!(
    //     NUM_VERTICES, NUM_VERTICES,
    //     [
    //         0, v2, v3, 0, v5,
    //         0, 0, v3, 0, v5,
    //         0, v2, 0, v4, 0,
    //         0, v2, 0, 0, v5,
    //         v1, 0, v3, 0, 0,
    //     ]
    // );
    //
    // #[rustfmt::skip]
    // let mut v = symbolic_matrix!(
    //     NUM_VERTICES, NUM_VERTICES,
    //     [
    //         0, 1, 1, 0, 1,
    //         0, 0, 1, 0, 1,
    //         0, 1, 0, 1, 0,
    //         0, 1, 0, 0, 1,
    //         1, 0, 1, 0, 0,
    //     ]
    // );
    if let Ok((m, mut v, labels)) = graphviz_io::parse_dot_file("./res/graph_11.dot") {
        for _ in 0..labels.len() - 1 {
            v = m.clone() * v;
        }

        let diag = v.diagonal();
        // print!("{:#?}", diag);
        let mut data = vec![];
        for i in 0..labels.len() {
            let val = diag[i as usize].clone();
            data.push(val);
        }

        match clean_up_data(&data, &labels) {
            Ok(data) => pretty_print_hamiltonian_paths(&data, &labels),
            Err(msg) => eprintln!("{}", msg),
        }
    }
}
