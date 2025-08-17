use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Copy, Clone)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "( {}, {} )\n( {}, {} )", self.0, self.1, self.2, self.3)
    }
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
}

fn transpose (m: Matrix) -> Matrix{
    Matrix(m.0, m.2, m.1, m.3)
}

pub fn tuples_demo() {

    let p1 = (1, true);
    println!("\n----------pair----------");
    println!("pair: {:?}",p1);
    println!("reversed_pair: {:?}",reverse(p1));
    println!("pair: {:?}",p1);


    let m1 = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("\n----------matrix----------");
    println!("\nDebug formatting\n{:?}", m1);
    println!("\nDisplay formatting\n{}", m1);


    println!("\n----------matrix transposed----------");
    let transposed_m1 = transpose(m1);
    println!("{}", transposed_m1);
}