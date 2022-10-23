use ham_cycles::symbolic_alg::VecSymbol;
use nalgebra::*;

fn main() {
    use num_traits::Zero;
    use num_traits::One;
    let m1 = DMatrix::from_row_slice(5, 5, &[
        VecSymbol::zero(), VecSymbol::zero(), VecSymbol::new("c"), VecSymbol::new("d"), VecSymbol::zero(),
        VecSymbol::zero(), VecSymbol::zero(), VecSymbol::zero(), VecSymbol::new("d"), VecSymbol::zero(),
        VecSymbol::zero(), VecSymbol::new("b"), VecSymbol::zero(), VecSymbol::zero(), VecSymbol::zero(),
        VecSymbol::zero(), VecSymbol::zero(), VecSymbol::new("c"), VecSymbol::zero(), VecSymbol::new("e"),
        VecSymbol::new("a"), VecSymbol::new("b"), VecSymbol::zero(), VecSymbol::zero(), VecSymbol::zero(),
    ]);

    // println!("{:#?}", m1);

    let v = DVector::from_row_slice(&[VecSymbol::zero(), VecSymbol::zero(), VecSymbol::one(), VecSymbol::zero(), VecSymbol::one()]);

    print!("{:#?}", m1 * v);
}
