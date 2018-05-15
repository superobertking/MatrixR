# Matrix

Simplified Rust version of Matrix (Last update: 2018.04.29)

Created for ShanghaiTech SIST new students' manual. Also uploaded in [https://github.com/SIST-Manual/Matrix/tree/master/Rust](https://github.com/SIST-Manual/Matrix/tree/master/Rust).

## Goal

- Matrix calcluation
- Generics
- Standard operators, e.g., `+`

## Description

Matrices are widely used both in mathematics and computer sciences, such as game thoery and economics. In this homework, you will implement a module for matrix calculation.

## Background

- [Matrix Wiki](https://en.wikipedia.org/wiki/Matrix_(mathematics))

## Syntax of Matrices

- `Matrix` is defined in Backus-Naur Form, see [BNF Wiki](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_Form).
- In case you don't know matrix or linear algebra well, search on internet，it is not too diffcult to understand. Programs are used to solve real-world problems. However, to understand the problems, you may have to learn some domain knowledge related to the problems.

```ebnf
Matrix::= "[" Rows "]" | "[" "]"
Rows::= Row | Row ";" Rows
Row::= element | element "," Row
element::= integer | floating | generic types
```

## Howework Specification

Implement a struct `Matrix` with generic type parameter `T` for representing matrix and implement the following operations for the Matrix struct.

- Creating

  - Matrix instance can be created by calling `Matrix::new(num_row, num_col, &[data])`, implement new for Matrix with `Copy` trait. You do not need to check if the size of data matches the dimension.

  - Matrix instance can be created by parsing a string. Implement `FromStr` trait so that we can generate a matrix using `x = s.parse::<Matrix<T>>()`, where `s` is a string in **Matrix** syntax allowing white space `" "` and `T` is the type parameter for Matrix which is specified when calling `parse`. If the string `s` does not follow the **Matrix** syntax, return `Result::Err(ParseMatrixError)` where `ParseMatrixError` is a enum defined as follow.

  - Define a `enum` called ParseMatrixError as follow:

    ```rust
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum ParseMatrixError {
        WrongBracketFormat,
        ColumnsNotAligned,
        ParseNumberError,
    }
    ```

    If an error occurred when parsing the string, return one type of error in `ParseMatrixError`. When the first and the last character in the whitespace-trimmed string is not `[` and `]`, return `WrongBracketFormat`. If the matrix has different number of elements in all rows, return `ColumnsNotAligned`. If a number cannot be extracted from an element token, return `ParseNumberError`.


- Output

  - To output the matrix in **Matrix** syntax, implement `Display` trait for Matrix. There is no whitespace allowed in the output.

- Operators: `+`, `-`

  - Implement for Matrix with all generic types with the bound of `std::ops::Add<Output=T>` (or `Sub`) and `Copy`.

  - Panic if the dimensions of the two input matrices are not compatible for the operation.

  - You need to implement all functionalities for these two operators, e.g. doing addition between matrix & matrix, matrix & scalar.

  - You need to implement for both reference and non-reference operands.

    - Hint: possible implementations:

      ```rust
      impl<'a, T: Add<Output = T> + Copy> Add for &'a Matrix<T>
      impl<'a, T: Add<Output = T> + Copy> Add<Matrix<T>> for &'a Matrix<T>
      impl<'a, T: Add<Output = T> + Copy> Add<&'a Matrix<T>> for Matrix<T>
      impl<T: Add<Output = T> + Copy> Add for Matrix<T>
      ```

    - Hint: Try not to copy-pase your code everywhere. It is rather a horrible practice to write too many duplicated codes in programming. Implement one and make others call that one.

- Operator `*`

  - Specifications are based on `+` and `-`.
  - Mulplication between two matrices are different from that of addition or subtraction, so read Wikipedia carefully before writing code. You may need the trait bound of both `std::ops::Mul<Output=T>` and `std::ops::Add<Output=T>`.

- Operator `/`

  - Dividing between integers may cause rounding and thus lead to imprecise results. Therefore, convert all the input values to `f64` and your output should be `Matrix<f64>`.
  - You still need to implement for matrix & scalar and different reference types.

- Operator `==`

  - You can simply implement this by utilizing rust compiler and derive `PartialEq` or `Eq` trait on defining Matrix struct. No need to bother write one by yourself unless your implementation is special.

-  `is_identity`

  - This function checks if a matrix is an identity matrix.
  - Since you cannot know the identity value of a generic type, you only need to implement this for all the generic types: `i8 i16 i32 i64 isize u8 u16 u32 u64 usize f32 f64`.
    - Hint: Again, try not to copy-paste. Macro in rust is a useful tool in doing batch implementation.

- `is_square`

  - This function checks if a matrix is a square matrix. Implement this function for all generic type parameters.

- `transposition`

  - This function returns the transposition of a matrix. Implement this function for all generic type parameters.

- `index`

  - Implement `std::ops::Index` and `std::ops::IndexMut` traits. The index argument is of type `(usize, usize)`. Return a reference to the corresponding cell in the matrix. Implement this for all generic types.

## Example

```rust
let _x = " [1,2,3; 4,5,6; 7,8,9]".parse::<Matrix<i32>>(); // can have white space
assert_eq!(Ok(Matrix::new(3, 3, &[1,2,3,4,5,6,7,8,9])), _x);
// should be the same with another way
let mut x = _x.unwrap();
assert_eq!(format!("{}", x), "[1,2,3;4,5,6;7,8,9]");
// the output does not contain any white space

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

assert!(x.is_square());

assert!("[1,2,3,4; 0,1,4,0; 0,0,1,0; 0,0,0,1]".parse::<Matrix<i32>>().unwrap().is_square());

assert_eq!(x[(2,1)], 8);

x[(1,2)] = 0;
assert_eq!(format!("{}", x), "[1,2,3;4,5,0;7,8,9]");

let m = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
assert_eq!(m.transposition(), Matrix::new(3, 2, &[-2, 1, -1, 2, 0, 3]));
```

