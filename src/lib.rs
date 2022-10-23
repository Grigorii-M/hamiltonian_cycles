pub use symbolic_alg::*;
mod symbolic_alg;

// We have to put this here because rust_analyzer does not detect usage in macro definitions
#[allow(unused_imports)]
use num_traits::{One, Zero};
#[allow(unused_imports)]
use symbolic_alg::*;

#[macro_export]
macro_rules! symbolic_matrix {
    ($width: literal, $height: literal, [$($val: tt),+ $(,)?]) => {
        {
            let data = stringify!($($val),+).split(", ").map(|el| VecSymbol::new(el)).collect::<Vec<_>>();

            if data.len() > $width * $height {
                panic!("There is more data than the size of the matrix; matrix size: {}, data size: {}", $width * $height, data.len());
            } else if data.len() < $width * $height {
                panic!("There is less data than the size of the matrix; matrix size: {}, data size: {}", $width * $height, data.len());
            }

            nalgebra::DMatrix::from_row_slice($height, $width, data.as_slice())
        }
    }
}

#[macro_export]
macro_rules! symbolic_vector {
    ($($val: tt),+ $(,)?) => {
        {
            println!("{:?}", stringify!($($val),+));
            let data = stringify!($($val),+).split(", ").map(|el| VecSymbol::new(el)).collect::<Vec<_>>();

            nalgebra::DVector::from_row_slice(data.as_slice())
        }
    }
}

#[cfg(test)]
mod tests {
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
                VecSymbol::zero(), VecSymbol::new("a"),
                VecSymbol::one(), VecSymbol::new("ab")
            ]);

        assert_eq!(m, expected);
    }
    
    #[test]
    #[rustfmt::skip]
    fn test_symbolic_vector_macro() {
        let m = symbolic_vector!(1, abc, 0, d);

        let expected = DVector::from_row_slice(
            &[VecSymbol::one(), VecSymbol::new("abc"), VecSymbol::zero(), VecSymbol::new("d")]
        );
        assert_eq!(m, expected);
    }
}
