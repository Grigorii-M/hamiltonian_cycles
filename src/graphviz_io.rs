use crate::symbolic_alg::*;
use nalgebra::DMatrix;

use std::collections::HashSet;
use std::error::Error;

pub fn parse_dot_file(
    file_path: &str,
) -> Result<(DMatrix<Symbol>, DMatrix<Symbol>, Vec<String>), Box<dyn Error>> {
    let file_contents = std::fs::read_to_string(file_path)?;
    Ok(parse_dot(&file_contents))
}

fn parse_dot(contents: &str) -> (DMatrix<Symbol>, DMatrix<Symbol>, Vec<String>) {
    let mut labels = HashSet::new();
    let mut edges = vec![];

    contents
        .split("\n")
        .filter(|line| line.contains(" -> "))
        .for_each(|line| {
            let lbls = line
                .split(" -> ")
                .map(|el| el.replace(";", "").trim().to_string())
                .collect::<Vec<String>>();
            edges.push((lbls[0].clone(), lbls[1].clone()));
            lbls.iter().for_each(|lbl| {
                labels.insert(lbl.clone());
            });
        });

    let mut labels = labels.into_iter().collect::<Vec<_>>();
    labels.sort();
    let mut symbol_matrix = vec![Symbol::zero(); labels.len() * labels.len()];
    let mut mult_matrix = vec![Symbol::zero(); labels.len() * labels.len()];
    edges.into_iter().for_each(|(start, end)| {
        if let Some(start_index) = labels.iter().position(|el| *el == start) {
            if let Some(end_index) = labels.iter().position(|el| *el == end) {
                symbol_matrix[start_index * labels.len() + end_index] = Symbol::new(vec![&end]);
                mult_matrix[start_index * labels.len() + end_index] = Symbol::one();
            }
        }
    });

    (
        DMatrix::from_row_slice(labels.len(), labels.len(), symbol_matrix.as_slice()),
        DMatrix::from_row_slice(labels.len(), labels.len(), mult_matrix.as_slice()),
        labels,
    )
}

#[cfg(test)]
mod dot_tests {
    use super::*;
    use crate::symbolic_matrix;

    #[test]
    fn test_correct_symbol_matrix() {
        #[rustfmt::skip]
        let expected_symbol = symbolic_matrix!(
            5, 5,
            [
                0, v2, v3, 0, v5,
                0, 0, v3, 0, v5,
                0, v2, 0, v4, 0,
                0, v2, 0, 0, v5,
                v1, 0, v3, 0, 0,
            ]
        );

        let (symb, _, _) = parse_dot(
            r"
graph G
{
  v1 -> v2;
  v1 -> v3;
  v1 -> v5;

  v2 -> v3;
  v2 -> v5;

  v3 -> v2;
  v3 -> v4;
  
  v4 -> v2;
  v4 -> v5;

  v5 -> v1;
  v5 -> v3;
}",
        );

        assert_eq!(symb, expected_symbol);
    }

    #[test]
    fn test_correct_mul_matrix() {
        #[rustfmt::skip]
        let expected_mul = symbolic_matrix!(
            5, 5,
            [
                0, 1, 1, 0, 1,
                0, 0, 1, 0, 1,
                0, 1, 0, 1, 0,
                0, 1, 0, 0, 1,
                1, 0, 1, 0, 0,
            ]
        );

        let (_, mul, _) = parse_dot(
            r"
graph G
{
  v1 -> v2;
  v1 -> v3;
  v1 -> v5;

  v2 -> v3;
  v2 -> v5;

  v3 -> v2;
  v3 -> v4;
  
  v4 -> v2;
  v4 -> v5;

  v5 -> v1;
  v5 -> v3;
}",
        );

        assert_eq!(mul, expected_mul);
    }

    #[test]
    fn test_correct_labels() {
        let expected_labels = vec!["v1", "v2", "v3", "v4", "v5"];

        let (_, _, lbls) = parse_dot(
            r"
graph G
{
  v1 -> v2;
  v1 -> v3;
  v1 -> v5;

  v2 -> v3;
  v2 -> v5;

  v3 -> v2;
  v3 -> v4;
  
  v4 -> v2;
  v4 -> v5;

  v5 -> v1;
  v5 -> v3;
}",
        );

        assert_eq!(lbls, expected_labels);
    }
}
