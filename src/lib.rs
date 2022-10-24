mod symbolic_alg;
use std::collections::HashSet;

pub use symbolic_alg::*;

pub fn pretty_print_hamiltonian_paths(data: &Vec<Symbol>, labels: &Vec<&str>) {
    if data.len() != labels.len() {
        panic!(
            "Data array does not coincide with labels array: data.len() = {}, labels.len() = {}",
            data.len(),
            labels.len()
        );
    }

    data.iter().enumerate().for_each(|(i, expr)| {
        let label = labels[i];
        println!("Starting at {}:", label);
        expr.data.iter().for_each(|prod| {
            println!("\t{} {} {}", label, prod, label);
        });
        println!();
    });
}

pub fn clean_up_data(data: &Vec<Symbol>, labels: &Vec<&str>) -> Result<Vec<Symbol>, String> {
    if data.len() != labels.len() {
        return Err(format!(
            "Data array does not coincide with labels array: data.len() = {}, labels.len() = {}",
            data.len(),
            labels.len()
        ));
    }

    let data = data
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, mut expr)| {
            let label = labels[i];
            expr.data = expr
                .data
                .iter_mut()
                .filter(|prod| {
                    let mut seen_labels = HashSet::new();
                    !prod.contains(label)
                        && (prod.split(" ").fold(true, |acc, el| {
                            if seen_labels.contains(el) {
                                acc && false
                            } else {
                                seen_labels.insert(el);
                                acc && true
                            }
                        }))
                })
                .map(|el| el.clone())
                .collect();
            expr
        })
        .filter(|el| !el.data.is_empty())
        .collect();

    Ok(data)
}

#[cfg(test)]
mod cleanup_tests {
    use super::*;

    #[test]
    fn test_remove_nothing() {
        let data = vec![
            Symbol::new(vec!["b c"]),
            Symbol::new(vec!["c a"]),
            Symbol::new(vec!["a b"]),
        ];
        let labels = vec!["a", "b", "c"];

        let res = clean_up_data(&data, &labels);
        assert_eq!(
            res.unwrap(),
            vec![
                Symbol::new(vec!["b c"]),
                Symbol::new(vec!["c a"]),
                Symbol::new(vec!["a b"])
            ]
        );
    }

    #[test]
    fn test_remove_matching_label() {
        let data = vec![
            Symbol::new(vec!["a b c"]),
            Symbol::new(vec!["c a"]),
            Symbol::new(vec!["a b c"]),
        ];
        let labels = vec!["a", "b", "c"];

        let res = clean_up_data(&data, &labels);
        assert_eq!(res.unwrap(), vec![Symbol::new(vec!["c a"])]);
    }

    #[test]
    fn test_remove_duplicates() {
        let data = vec![
            Symbol::new(vec!["c b c"]),
            Symbol::new(vec!["c a c"]),
            Symbol::new(vec!["a b"]),
        ];
        let labels = vec!["a", "b", "c"];

        let res = clean_up_data(&data, &labels);
        assert_eq!(res.unwrap(), vec![Symbol::new(vec!["a b"])]);
    }

    #[test]
    fn test_complex() {
        let data = vec![
            Symbol::new(vec!["b c"]),
            Symbol::new(vec!["c a c"]),
            Symbol::new(vec!["a b c"]),
        ];
        let labels = vec!["a", "b", "c"];

        let res = clean_up_data(&data, &labels);
        assert_eq!(res.unwrap(), vec![Symbol::new(vec!["b c"])]);
    }
}
