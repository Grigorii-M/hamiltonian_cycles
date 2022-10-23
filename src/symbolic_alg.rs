use std::ops::*;

#[derive(Debug, Clone, PartialEq)]
pub struct VecSymbol {
    data: String,
}

impl VecSymbol {
    pub fn new(data: &str) -> Self {
        Self { data: data.to_string() }
    }
}

impl Add for VecSymbol {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let data = if self.data == "0" {
            rhs.data
        } else if rhs.data == "0" {
            self.data
        } else {
            format!("+ {} {}", self.data, rhs.data)
        };

        Self {
            data
        }
    }
}

impl AddAssign for VecSymbol {
    fn add_assign(&mut self, rhs: Self) {
        self.data = Self::add(self.clone(), rhs).data;
    }
}

impl Mul for VecSymbol {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let data = if self.data == "0" || rhs.data == "0" {
            "0".to_string()
        } else if self.data == "1" {
            rhs.data
        } else if rhs.data == "1" {
            self.data
        } else {
            format!("* {} {}", self.data, rhs.data)
        };

        Self {
            data
        }
    }
}

impl MulAssign for VecSymbol {
    fn mul_assign(&mut self, rhs: Self) {
        self.data = Self::mul(self.clone(), rhs).data;
    }
}

impl num_traits::Zero for VecSymbol {
    fn is_zero(&self) -> bool {
        self.data == "0"
    }

    fn zero() -> Self {
        Self { data: "0".to_string() }
    }
}

impl num_traits::One for VecSymbol {
    fn one() -> Self {
        Self { data: "1".to_string() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::*;

    #[test]
    fn test_add() {
        let v1 = VecSymbol::new("v1");
        let v2 = VecSymbol::new("v2");

        assert_eq!(v1 + v2, VecSymbol::new("+ v1 v2"))
    }

    #[test]
    fn test_add_assign() {
        let mut v1 = VecSymbol::new("v1");
        let v2 = VecSymbol::new("v2");
        v1 += v2;

        assert_eq!(v1, VecSymbol::new("+ v1 v2"))
    }

    #[test]
    fn test_mul() {
        let v1 = VecSymbol::new("v1");
        let v2 = VecSymbol::new("v2");

        assert_eq!(v1 * v2, VecSymbol::new("* v1 v2"))
    }

    #[test]
    fn test_mul_assign() {
        let mut v1 = VecSymbol::new("v1");
        let v2 = VecSymbol::new("v2");
        v1 *= v2;

        assert_eq!(v1, VecSymbol::new("* v1 v2"))
    }

    #[test]
    fn test_mul_zero() {
        let v1 = VecSymbol::new("0");
        let v2 = VecSymbol::new("v2");

        assert_eq!(v1 * v2, VecSymbol::new("0"));

        let v1 = VecSymbol::new("0");
        let v2 = VecSymbol::new("v2");
        assert_eq!(v2 * v1, VecSymbol::new("0"));
    }

    #[test]
    fn test_add_zero() {
        let v1 = VecSymbol::new("0");
        let v2 = VecSymbol::new("v2");

        assert_eq!(v1 + v2, VecSymbol::new("v2"));

        let v1 = VecSymbol::new("0");
        let v2 = VecSymbol::new("v2");
        assert_eq!(v2 + v1, VecSymbol::new("v2"));
    }

    #[test]
    fn test_mul_one() {
        let v1 = VecSymbol::new("1");
        let v2 = VecSymbol::new("v2");

        assert_eq!(v1 * v2, VecSymbol::new("v2"));

        let v1 = VecSymbol::new("1");
        let v2 = VecSymbol::new("v2");
        assert_eq!(v2 * v1, VecSymbol::new("v2"));
    }

    #[test]
    fn test_add_one() {
        let v1 = VecSymbol::new("1");
        let v2 = VecSymbol::new("v2");

        assert_eq!(v1 + v2, VecSymbol::new("+ 1 v2"));

        let v1 = VecSymbol::new("1");
        let v2 = VecSymbol::new("v2");
        assert_eq!(v2 + v1, VecSymbol::new("+ v2 1"));
    }

    #[test]
    fn test_mat_mul() {
        let m1 = DMatrix::from_element(2, 2, VecSymbol::new("v1"));
        let m2 = DMatrix::from_element(2, 2, VecSymbol::new("v2"));
        let got = m1 * m2;
        let expected = Matrix2::from_element(VecSymbol::new("+ * v1 v2 * v1 v2"));

        assert_eq!(got, expected);
    }

    #[test]
    fn test_mat_complex_mul() {
        use num_traits::Zero;
        use num_traits::One;
        let m1 = DMatrix::from_row_slice(3, 3, &[
            VecSymbol::zero(), VecSymbol::new("v2"), VecSymbol::zero(),
            VecSymbol::zero(), VecSymbol::zero(), VecSymbol::new("v3"),
            VecSymbol::new("v1"), VecSymbol::zero(), VecSymbol::zero(),
        ]);

        let m2 = DMatrix::from_row_slice(3, 3, &[
            VecSymbol::zero(), VecSymbol::one(), VecSymbol::zero(),
            VecSymbol::zero(), VecSymbol::zero(), VecSymbol::one(),
            VecSymbol::one(), VecSymbol::zero(), VecSymbol::zero(),
        ]);
        let got = m1 * m2;
        let expected = Matrix2::from_element(VecSymbol::new("+ * v1 v2 * v1 v2"));

        assert_eq!(got, expected);
    }
}
