use std::ops::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    /// Each vector element is a multiplication, the whole vector is addition
    pub(crate) data: Vec<String>,
}

impl Symbol {
    pub fn new(data: Vec<&str>) -> Self {
        let data = data.into_iter().map(|el| el.to_string()).collect();
        Self { data }
    }
}

impl Add for Symbol {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = self.data.clone();
        data.extend(rhs.data);
        data = data.into_iter().filter(|el| *el != "0").collect();

        if data.is_empty() {
            use num_traits::Zero;
            Self::zero()
        } else {
            Self { data }
        }
    }
}

impl AddAssign for Symbol {
    fn add_assign(&mut self, rhs: Self) {
        self.data = Self::add(self.clone(), rhs).data;
    }
}

impl Mul for Symbol {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let data = self
            .data
            .into_iter()
            .flat_map(|l_el| {
                let d = rhs.data.iter().map(move |r_el| {
                    if l_el != "0" && r_el != "0" {
                        if l_el == "1" {
                            r_el.clone()
                        } else if r_el == "1" {
                            l_el.clone()
                        } else {
                            let mut el = l_el.clone();
                            el.push(' ');
                            el.push_str(&r_el);
                            el
                        }
                    } else {
                        "0".to_string()
                    }
                });
                d
            })
            .filter(|el| el != "0")
            .collect::<Vec<String>>();

        if data.is_empty() {
            use num_traits::Zero;
            Self::zero()
        } else {
            Self { data }
        }
    }
}

impl MulAssign for Symbol {
    fn mul_assign(&mut self, rhs: Self) {
        self.data = Self::mul(self.clone(), rhs).data;
    }
}

impl num_traits::Zero for Symbol {
    fn is_zero(&self) -> bool {
        self.data.len() == 1 && self.data[0] == "0"
    }

    fn zero() -> Self {
        Self {
            data: vec!["0".to_string()],
        }
    }
}

impl num_traits::One for Symbol {
    fn one() -> Self {
        Self {
            data: vec!["1".to_string()],
        }
    }
}

#[cfg(test)]
mod data_tests {
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Symbol::new(vec!["v1"]);
        let v2 = Symbol::new(vec!["v2"]);

        assert_eq!(v1 + v2, Symbol::new(vec!["v1", "v2"]))
    }

    #[test]
    fn test_add_assign() {
        let mut v1 = Symbol::new(vec!["v1"]);
        let v2 = Symbol::new(vec!["v2"]);
        v1 += v2;

        assert_eq!(v1, Symbol::new(vec!["v1", "v2"]))
    }

    #[test]
    fn test_mul() {
        let v1 = Symbol::new(vec!["v1"]);
        let v2 = Symbol::new(vec!["v2"]);

        assert_eq!(v1 * v2, Symbol::new(vec!["v1 v2"]))
    }

    #[test]
    fn test_mul_assign() {
        let mut v1 = Symbol::new(vec!["v1"]);
        let v2 = Symbol::new(vec!["v2"]);
        v1 *= v2;

        assert_eq!(v1, Symbol::new(vec!["v1 v2"]))
    }

    #[test]
    fn test_mul_zero() {
        let v1 = Symbol::new(vec!["0"]);
        let v2 = Symbol::new(vec!["v2"]);

        assert_eq!(v1 * v2, Symbol::new(vec!["0"]));

        let v1 = Symbol::new(vec!["0"]);
        let v2 = Symbol::new(vec!["v2"]);
        assert_eq!(v2 * v1, Symbol::new(vec!["0"]));
    }

    #[test]
    fn test_mul_several_zeros() {
        let v1 = Symbol::new(vec!["0"]);
        let v2 = Symbol::new(vec!["0"]);

        use num_traits::Zero;
        assert_eq!(v1 * v2, Symbol::zero());
    }

    #[test]
    fn test_add_zero() {
        let v1 = Symbol::new(vec!["0"]);
        let v2 = Symbol::new(vec!["v2"]);

        assert_eq!(v1 + v2, Symbol::new(vec!["v2"]));

        let v1 = Symbol::new(vec!["0"]);
        let v2 = Symbol::new(vec!["v2"]);
        assert_eq!(v2 + v1, Symbol::new(vec!["v2"]));
    }

    #[test]
    fn test_add_several_zeros() {
        let v1 = Symbol::new(vec!["0"]);
        let v2 = Symbol::new(vec!["0"]);

        use num_traits::Zero;
        assert_eq!(v1 + v2, Symbol::zero());
    }

    #[test]
    fn test_mul_one() {
        let v1 = Symbol::new(vec!["1"]);
        let v2 = Symbol::new(vec!["v2"]);

        assert_eq!(v1 * v2, Symbol::new(vec!["v2"]));

        let v1 = Symbol::new(vec!["1"]);
        let v2 = Symbol::new(vec!["v2"]);
        assert_eq!(v2 * v1, Symbol::new(vec!["v2"]));
    }

    #[test]
    fn test_add_one() {
        let v1 = Symbol::new(vec!["1"]);
        let v2 = Symbol::new(vec!["v2"]);

        assert_eq!(v1 + v2, Symbol::new(vec!["1", "v2"]));

        let v1 = Symbol::new(vec!["1"]);
        let v2 = Symbol::new(vec!["v2"]);
        assert_eq!(v2 + v1, Symbol::new(vec!["v2", "1"]));
    }

    #[test]
    fn test_several_add() {
        let v1 = Symbol::new(vec!["1"]);
        let v2 = Symbol::new(vec!["v2"]);
        let v3 = Symbol::new(vec!["v3"]);

        assert_eq!(v1 + v2 + v3, Symbol::new(vec!["1", "v2", "v3"]));
    }

    #[test]
    fn test_several_mul() {
        let v1 = Symbol::new(vec!["1"]);
        let v2 = Symbol::new(vec!["v2"]);
        let v3 = Symbol::new(vec!["v3"]);

        assert_eq!(v1 * v2 * v3, Symbol::new(vec!["v2 v3"]));
    }

    #[test]
    fn test_add_and_mul() {
        let v1 = Symbol::new(vec!["1"]);
        let v2 = Symbol::new(vec!["v2"]);
        let v3 = Symbol::new(vec!["v3"]);

        assert_eq!(v1 + v2 * v3, Symbol::new(vec!["1", "v2 v3"]));

        let v1 = Symbol::new(vec!["1"]);
        let v2 = Symbol::new(vec!["v2"]);
        let v3 = Symbol::new(vec!["v3"]);

        assert_eq!(v1 * v2 + v3, Symbol::new(vec!["v2", "v3"]));
    }

    #[test]
    fn test_mul_of_sum() {
        let v1 = Symbol::new(vec!["v1"]);
        let v2 = Symbol::new(vec!["v2"]);
        let v3 = Symbol::new(vec!["v3"]);
        let v4 = Symbol::new(vec!["v4"]);

        assert_eq!(
            (v1 + v2) * (v3 + v4),
            Symbol::new(vec!["v1 v3", "v1 v4", "v2 v3", "v2 v4"])
        );
    }

    #[test]
    fn test_sum_of_mul() {
        let v1 = Symbol::new(vec!["v1"]);
        let v2 = Symbol::new(vec!["v2"]);
        let v3 = Symbol::new(vec!["v3"]);
        let v4 = Symbol::new(vec!["v4"]);

        assert_eq!(v1 * v2 + v3 * v4, Symbol::new(vec!["v1 v2", "v3 v4"]));
    }

    use nalgebra::*;
    #[test]
    fn test_mat_mul() {
        let m1 = DMatrix::from_element(2, 2, Symbol::new(vec!["v1"]));
        let m2 = DMatrix::from_element(2, 2, Symbol::new(vec!["v2"]));
        let got = m1 * m2;
        let expected = Matrix2::from_element(Symbol::new(vec!["v1 v2", "v1 v2"]));

        assert_eq!(got, expected);
    }
}
