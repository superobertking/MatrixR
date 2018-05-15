use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut};
use std::str::FromStr;


#[derive(PartialEq, Debug)]
pub struct Matrix<T> {
    /// Stores elements in [row-major order](https://en.wikipedia.org/wiki/Row-major_order)
    data: Vec<T>,
    /// Number of rows
    row: usize,
    /// Number of columns
    col: usize,
}

impl<T> Matrix<T> {
    /// Returns the number of rows and columns in the first and second
    /// elements of the tuple, respectively.
    pub fn shape(&self) -> (usize, usize) {
        (self.row, self.col)
    }
    /// Return whether the row and col numbers of this matrix are the same.
    pub fn is_square(&self) -> bool {
        self.row == self.col
    }
}

impl<T: Copy> Matrix<T> {
    /// Creates a new matrix of `row` rows and `col` columns, and initializes
    /// the matrix with the elements in `values` in row-major order.
    pub fn new(row: usize, col: usize, values: &[T]) -> Matrix<T> {
        Matrix { data: values.to_vec(), row: row, col: col }
    }
    /// Transpose matrix
    pub fn transposition(&self) -> Matrix<T> {
        let mut data = Vec::new();
        for i in 0..self.col {
            for j in 0..self.row {
                data.push(self.data[j * self.col + i]);
            }
        }
        Matrix { data, row: self.col, col: self.row }
    }
}


/// Implement is_idenity for primitive types.
macro_rules! impl_is_identity {
    ($($T: ty)*; $zero: expr, $identity: expr) => {$(
        impl Matrix<$T> {
            pub fn is_identity(&self) -> bool {
                if !self.is_square() {
                    return false
                }
                let mut idx = 0;
                for i in 0..self.row {
                    for j in 0..self.col {
                        if i == j && self.data[idx] != $identity || i != j && self.data[idx] != $zero {
                            return false
                        }
                        idx += 1;
                    }
                }
                true
            }
        }
    )*}
}

impl_is_identity!(i8 i16 i32 i64 isize u8 u16 u32 u64 usize; 0, 1);
impl_is_identity!(f32 f64; 0.0, 1.0);


/// Implement Display trait for Matrix
impl<T: fmt::Display> fmt::Display for Matrix<T> {
    /// Outputs using `write!(f, ...)`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..self.data.len() {
            write!(f, "{}{}", self.data[i],
                [",", ";", ""][((i + 1) % self.col == 0) as usize + (i == self.data.len() - 1) as usize]
            )?;
        }
        write!(f, "]")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseMatrixError {
    WrongBracketFormat,
    ColumnsNotAligned,
    ParseNumberError,
}

/// Parse Matrix from string
impl<T: FromStr> FromStr for Matrix<T> {
    type Err = ParseMatrixError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Trim leading and trailing spaces.
        let s = s.trim().to_string();
        // Check bracket format
        if s.len() < 2 || &s[0..1] != "[" || &s[(s.len() - 1)..s.len()] != "]" {
            return Err(ParseMatrixError::WrongBracketFormat);
        }
        // Get real content
        let s = s[1..(s.len() - 1)].to_string();
        let (mut num_row, mut num_col) = (0, 0);
        let mut data = vec![];
        // Iterate over rows
        for (idx, row_string) in s.split(';').enumerate() {
            let row_elements = row_string.split(',').collect::<Vec<&str>>();
            // Check column alignment
            if idx == 0 {
                num_col = row_elements.len();
            } else if num_col != row_elements.len() {
                return Err(ParseMatrixError::ColumnsNotAligned);
            }
            num_row += 1;
            // Append parse element to data
            for element in row_elements {
                if let Ok(x) = element.trim().parse::<T>() {
                    data.push(x);
                } else {
                    return Err(ParseMatrixError::ParseNumberError);
                }
            }
        }
        Ok(Matrix { data, row: num_row, col: num_col })
    }
}


impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 * self.col + index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 * self.col + index.1]
    }
}


impl<'a, T: Add<Output = T> + Copy> Add for &'a Matrix<T> {
    type Output = Matrix<T>;
    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn add(self, rhs: Self) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col {
            panic!();
        }
        Matrix {
            data: self.data.iter()
                .zip(rhs.data.iter())
                .map(|x| *x.0 + *x.1)
                .collect::<Vec<T>>(),
            row: self.row,
            col: self.col,
        }
    }
}

impl<'a, T: Add<Output = T> + Copy> Add<&'a T> for &'a Matrix<T> {
    type Output = Matrix<T>;
    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn add(self, rhs: &'a T) -> Self::Output {
        Matrix {
            data: self.data.iter().map(|x| *x + *rhs).collect::<Vec<T>>(),
            row: self.row,
            col: self.col,
        }
    }
}

impl<'a, T: Sub<Output = T> + Copy> Sub for &'a Matrix<T> {
    type Output = Matrix<T>;
    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn sub(self, rhs: Self) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col {
            panic!();
        }
        Matrix {
            data: self.data.iter()
                .zip(rhs.data.iter())
                .map(|x| *x.0 - *x.1)
                .collect::<Vec<T>>(),
            row: self.row,
            col: self.col,
        }
    }
}

impl<'a, T: Sub<Output = T> + Copy> Sub<&'a T> for &'a Matrix<T> {
    type Output = Matrix<T>;
    fn sub(self, rhs: &'a T) -> Self::Output {
        Matrix {
            data: self.data.iter().map(|x| *x - *rhs).collect::<Vec<T>>(),
            row: self.row,
            col: self.col,
        }
    }
}

impl<'a, T: Neg<Output = T> + Copy> Neg for &'a Matrix<T> {
    type Output = Matrix<T>;
    fn neg(self) -> Self::Output {
        Matrix {
            data: self.data.iter().map(|x| -*x).collect::<Vec<T>>(),
            row: self.row,
            col: self.col,
        }
    }
}

impl<'a, T: Neg<Output = T> + Copy> Neg for Matrix<T> {
    type Output = Matrix<T>;
    fn neg(self) -> Self::Output {
        -&self
    }
}

impl<'a, T: Add<Output = T> + Mul<Output = T> + Copy> Mul for &'a Matrix<T> {
    type Output = Matrix<T>;
    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: Self) -> Self::Output {
        if self.col != rhs.row {
            panic!();
        }
        let mut data = Vec::new();
        for i in 0..self.row {
            for j in 0..rhs.col {
                data.push(self.data[i * self.col] * rhs.data[j]);
                for k in 1..self.col {
                    data[i * rhs.col + j] = data[i * rhs.col + j] +
                        self.data[i * self.col + k] * rhs.data[k * rhs.col + j];
                }
            }
        }
        Matrix { data, row: self.row, col: rhs.col }
    }
}

impl<'a, T: Mul<Output = T> + Copy> Mul<&'a T> for &'a Matrix<T> {
    type Output = Matrix<T>;
    /// Returns the multiplication of `self` by `rhs`, where `rhs` is a scalar.
    fn mul(self, rhs: &'a T) -> Self::Output {
        Matrix {
            data: self.data.iter().map(|x| *x * *rhs).collect::<Vec<T>>(),
            row: self.row,
            col: self.col,
        }
    }
}


impl<'a, T: Copy + Into<f64>> Div for &'a Matrix<T> {
    type Output = Matrix<f64>;
    /// Returns the division of `self` by `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn div(self, rhs: Self) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col {
            panic!();
        }
        Matrix {
            data: self.data.iter()
                .zip(rhs.data.iter())
                .map(|x| (*x.0).into() / (*x.1).into())
                .collect::<Vec<f64>>(),
            row: self.row,
            col: self.col,
        }
    }
}

impl<'a, T: Copy + Into<f64>> Div<&'a T> for &'a Matrix<T> {
    type Output = Matrix<f64>;
    /// Returns the division of `self` by `rhs`, where `rhs` is a scalar.
    fn div(self, rhs: &'a T) -> Self::Output {
        Matrix {
            data: self.data.iter()
                .map(|x| (*x).into() / (*rhs).into())
                .collect::<Vec<f64>>(),
            row: self.row,
            col: self.col,
        }
    }
}


/// Implement Oprators with other reference scenarios
/// Referenced macro usage:
/// https://users.rust-lang.org/t/how-to-create-a-macro-to-impl-a-provided-type-parametrized-trait/5289/3
macro_rules! impl_op_refs {
    (
        $Op:ident, $op:ident, $Rhs:ident < $($params:ident),* $(,)* >, $Out:ident;
        $($traits_vals:ident < $($keys:ident = $values:ident),* $(,)* >),*;
        $($traits:ident < $($args:ident),* $(,)* >),*
    ) => {
        // Implement operators for (M, &M), (&M, M) and (M, M)
        impl<'a, T: Copy $(+ $traits_vals<$($keys = $values),*>)* $(+ $traits<$($args),*>)*> $Op<&'a $Rhs <$($params),*> > for Matrix<T> {
            type Output = Matrix<$Out>;
            fn $op(self, rhs: &'a $Rhs <$($params),*>) -> Self::Output {
                (&self).$op(rhs)
            }
        }
        impl<'a, T: Copy $(+ $traits_vals<$($keys = $values),*>)* $(+ $traits<$($args),*>)*> $Op<$Rhs <$($params),*> > for &'a Matrix<T> {
            type Output = Matrix<$Out>;
            fn $op(self, rhs: $Rhs <$($params),*>) -> Self::Output {
                self.$op(&rhs)
            }
        }
        impl<'a, T: Copy $(+ $traits_vals<$($keys = $values),*>)* $(+ $traits<$($args),*>)*> $Op<$Rhs <$($params),*> > for Matrix<T> {
            type Output = Matrix<$Out>;
            fn $op(self, rhs: $Rhs <$($params),*>) -> Self::Output {
                (&self).$op(&rhs)
            }
        }
    }
}


impl_op_refs!(Add, add, Matrix<T>, T; Add<Output=T>; );
impl_op_refs!(Sub, sub, Matrix<T>, T; Sub<Output=T>; );
impl_op_refs!(Mul, mul, Matrix<T>, T; Mul<Output=T>, Add<Output=T>; );
impl_op_refs!(Div, div, Matrix<T>, f64; ; Into<f64>);
impl_op_refs!(Add, add, T<>, T; Add<Output=T>; );
impl_op_refs!(Sub, sub, T<>, T; Sub<Output=T>; );
impl_op_refs!(Mul, mul, T<>, T; Mul<Output=T>; );
impl_op_refs!(Div, div, T<>, f64; ; Into<f64>);


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::Matrix;
        let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
        let y = Matrix::new(2, 3, &[1, 2, 3, 4, 5, 6]);
        assert_eq!(&x + &y - &y, x);
        assert_eq!(format!("{}", x), "[-2,-1,0;1,2,3]");
        let z = "[-2,-1,0;1,2,3]".parse::<Matrix<i32>>().unwrap();
        assert_eq!(x, z);
        assert_eq!(x.transposition(), Matrix::new(3, 2, &[-2, 1, -1, 2, 0, 3]));
    }
    #[test]
    fn sample() {
        use super::{Matrix, ParseMatrixError};

        let _x = " [1,2,3; 4,5,6; 7,8,9]".parse::<Matrix<i32>>();
        assert_eq!(Ok(Matrix::new(3, 3, &[1,2,3,4,5,6,7,8,9])), _x);
        let mut x = _x.unwrap();
        assert_eq!(format!("{}", x), "[1,2,3;4,5,6;7,8,9]");

        let mut t;
        t = "1,2,3; 4,5,6; 7,8,9]".parse::<Matrix<i32>>();
        assert_eq!(Err(ParseMatrixError::WrongBracketFormat), t);

        t = "[1,2,x; 4,5,6; 7,8,9]".parse::<Matrix<i32>>();
        assert_eq!(Err(ParseMatrixError::ParseNumberError), t);

        t = "[1,2,; 4,5,6; 7,8,9]".parse::<Matrix<i32>>();
        assert_eq!(Err(ParseMatrixError::ParseNumberError), t);

        t = "[1,2; 4,5,6; 7,8,9]".parse::<Matrix<i32>>();
        assert_eq!(Err(ParseMatrixError::ColumnsNotAligned), t);

        let y = "[0,1,2; 3,4,5; 6,7,8]".parse::<Matrix<i32>>().unwrap();
        let mut z;
        z = &x + &y;
        assert_eq!(format!("{}", z), "[1,3,5;7,9,11;13,15,17]");

        z = &x - &y;
        assert_eq!(format!("{}", z), "[1,1,1;1,1,1;1,1,1]");

        z = &x * 2;
        assert_eq!(format!("{}", z), "[2,4,6;8,10,12;14,16,18]");

        z = &x * &y;
        assert_eq!(format!("{}", z), "[24,30,36;51,66,81;78,102,126]");

        assert_eq!(format!("{}", &z / 2), "[12,15,18;25.5,33,40.5;39,51,63]");

        assert_eq!(x == y, false);

        assert!(x == "[1,2,3; 4,5,6;7,8,9]".parse::<Matrix<i32>>().unwrap());

        assert_eq!(x.is_identity(), false);

        assert!("[1,0,0,0; 0,1,0,0; 0,0,1,0; 0,0,0,1]".parse::<Matrix<f64>>().unwrap().is_identity());

        // aaa

        assert!(x.is_square());

        assert!("[1,2,3,4; 0,1,4,0; 0,0,1,0; 0,0,0,1]".parse::<Matrix<i32>>().unwrap().is_square());

        assert_eq!(x[(2,1)], 8);

        x[(1,2)] = 0;
        assert_eq!(format!("{}", x), "[1,2,3;4,5,0;7,8,9]");

        let m = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
        assert_eq!(m.transposition(), Matrix::new(3, 2, &[-2, 1, -1, 2, 0, 3]));
    }
}
