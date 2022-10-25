mod data;
pub use data::*;
pub use num_traits::{One, Zero};

// We have to put this here because rust_analyzer does not detect usage in macro definitions
#[macro_export]
macro_rules! symbolic_matrix {
    ($width: literal, $height: literal, [$($val: tt),+ $(,)?]) => {
        {
            let data = stringify!($($val),+)
                .split("\n")
                .collect::<Vec<_>>()
                .join(" ")
                .split(", ")
                .map(|el| Symbol::new(vec![el]))
                .collect::<Vec<_>>();

            if data.len() > $width * $height {
                panic!("There is more data than the size of the matrix; matrix size: {}, data size: {}", $width * $height, data.len());
            } else if data.len() < $width * $height {
                println!("{:?}", data);
                panic!("There is less data than the size of the matrix; matrix size: {}, data size: {}", $width * $height, data.len());
            }

            nalgebra::DMatrix::from_row_slice($height, $width, data.as_slice())
        }
    };
    ($width: ident, $height: ident, [$($val: tt),+ $(,)?]) => {
        {
            let data = stringify!($($val),+)
                .split("\n")
                .collect::<Vec<_>>()
                .join(" ")
                .split(", ")
                .map(|el| Symbol::new(vec![el]))
                .collect::<Vec<_>>();

            if data.len() > $width as usize * $height as usize {
                panic!("There is more data than the size of the matrix; matrix size: {}, data size: {}", $width * $height, data.len());
            } else if data.len() < $width as usize * $height as usize {
                println!("{:?}", data);
                panic!("There is less data than the size of the matrix; matrix size: {}, data size: {}", $width * $height, data.len());
            }

            nalgebra::DMatrix::from_row_slice($height as usize, $width as usize, data.as_slice())
        }
    }
}

#[macro_export]
macro_rules! symbolic_vector {
    ($($val: tt),+ $(,)?) => {
        {
            let data = stringify!($($val),+)
                .split(", ")
                .map(|el| Symbol::new(vec![el]))
                .collect::<Vec<_>>();

            nalgebra::DVector::from_row_slice(data.as_slice())
        }
    }
}

#[cfg(test)]
mod macro_tests {
    use super::*;
    use nalgebra::*;
    use num_traits::One;
    use num_traits::Zero;

    #[test]
    #[rustfmt::skip]
    fn test_symbolic_matrix_macro() {
        let m = symbolic_matrix!(
            2, 2,
            [
                0, a,
                1, ab,
            ]
        );

        let expected = DMatrix::from_row_slice(
            2, 2, 
            &[
                Symbol::zero(), Symbol::new(vec!["a"]),
                Symbol::one(), Symbol::new(vec!["ab"])
            ]);

        assert_eq!(m, expected);
    }

    #[test]
    #[rustfmt::skip]
    fn test_symbolic_vector_macro() {
        let m = symbolic_vector!(1, abc, 0, d);

        let expected = DVector::from_row_slice(
            &[Symbol::one(), Symbol::new(vec!["abc"]), Symbol::zero(), Symbol::new(vec!["d"])]
        );
        assert_eq!(m, expected);
    }
}
