use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign};


trait One
{
    fn one() -> Self;
}


impl One for f64
{
    fn one() -> f64
    {
        1.0
    }
}


impl One for f32
{
    fn one() -> f32
    {
        1.0
    }
}


impl One for i32
{
    fn one() -> i32
    {
        1
    }
}


#[derive(Eq, PartialEq, Hash, Debug)]
enum DecomposedMatrix
{
    L,
    U
}


#[derive(Debug)]
struct Matrix<T>
{
    elements: Vec<Vec<T>>
}


impl<T> Matrix<T>
    where T: One + Copy + Default +
             Add<Output = T> + Mul<Output = T> +
             AddAssign + MulAssign
{
    fn multiply(&self, other: &Matrix<T>) -> Result<Matrix<T>, &str>
    {
        if self.elements[0].len() != other.elements.len()
        {
            return Err("cannot multiply matrices!");
        }

        let mut elements = Vec::new();
        for i in 0..self.elements.len()
        {
            let mut current_row: Vec<T> = Vec::new();
            for j in 0..other.elements[0].len()
            {
                let mut current_element: T = Default::default();
                for k in 0..self.elements[0].len()
                {
                    current_element += self.elements[i][k] * other.elements[k][j];
                }
                current_row.push(current_element);
            }
            elements.push(current_row);
        }
        Ok(Matrix { elements })
    }


    fn sum(&self, other: &Matrix<T>) -> Result<Matrix<T>, &str>
    {
        if (self.elements[0].len() != other.elements[0].len()) &&
            (self.elements.len() != other.elements.len())
        {
            return Err("cannot sum matrices!");
        }
        let mut elements = Vec::new();
        for i in 0..self.elements.len()
        {
            let mut current_row: Vec<T> = Vec::new();
            for j in 0..other.elements[0].len()
            {
                let current_element = self.elements[i][j] + other.elements[i][j];
                current_row.push(current_element);
            }
            elements.push(current_row);
        }
        Ok(Matrix { elements })
    }


    fn _convert_elements<V>(&self) -> Matrix<V>
        where V: From<T>
    {
        let mut elements = Vec::new();
        for i in 0..self.elements.len()
        {
            let mut current_row = Vec::new();
            {
                for j in 0..self.elements[0].len()
                {
                    let current_element = self.elements[i][j].into();
                    current_row.push(current_element);
                }
            }
            elements.push(current_row);
        }
        Matrix { elements }
    }


    fn decompose_to_l_u<V>(&self) -> Result<HashMap<DecomposedMatrix, Matrix<V>>, &str>
        where V: From<T> + Default + One + Copy +
                 Sub<Output = V> + Mul<Output = V> +
                 Div<Output = V>
    {
        if (self.elements.len() != self.elements[0].len()) || self.elements.len() < 2
        {
            return Err("cannot decompose matrix!");
        }
        let mut lower_matrix_elements = Vec::new();
        for i in 0..self.elements.len()
        {
            let mut current_row = Vec::new();
            for j in 0..self.elements.len()
            {
                if i == j
                {
                    current_row.push(One::one());
                }
                else
                {
                    current_row.push(Default::default());
                }
            }
            lower_matrix_elements.push(current_row);
        }
        let mut upper_matrix_elements = self._convert_elements::<V>().elements;
        let mut row_number = 0;
        while row_number < upper_matrix_elements.len() - 1
        {
            for i in (row_number + 1)..upper_matrix_elements.len()
            {
                let mut current_row= Vec::new();
                let current_coeff =
                    upper_matrix_elements[i][row_number] /
                    upper_matrix_elements[row_number][row_number];
                lower_matrix_elements[i][row_number] = current_coeff;
                for j in 0..upper_matrix_elements[0].len()
                {
                    let current_element =
                        (
                            upper_matrix_elements[i][j] -
                            upper_matrix_elements[row_number][j] * current_coeff
                        ).into();
                    current_row.push(current_element);
                }
                upper_matrix_elements[i] = current_row;
            }
            row_number += 1;
        }
        let mut result = HashMap::new();
        result.insert(DecomposedMatrix::L, Matrix { elements: lower_matrix_elements });
        result.insert(DecomposedMatrix::U, Matrix { elements: upper_matrix_elements });
        Ok(result)
    }


    fn determinant_u(&self) -> T
    {
        let mut determinant = One::one();
        for i in 0..self.elements.len()
        {
            determinant *= self.elements[i][i]
        }
        determinant
    }


    fn transpose(&self) -> Matrix<T>
    {
        let mut transposed_elements = Vec::new();
        for j in 0..self.elements[0].len()
        {
            let mut transposed_row = Vec::new();
            for i in 0..self.elements.len()
            {
                transposed_row.push(self.elements[i][j]);
            }
            transposed_elements.push(transposed_row);
        }
        Matrix { elements: transposed_elements }
    }


    fn solve_equations<V, W>(&self, other: Matrix<W>) -> Result<Matrix<V>, &str>
        where V: From<T> + From<W> + Default + One + Copy +
                 Sub<Output = V> + Mul<Output = V> +
                 Div<Output = V> + SubAssign,
              W: Default + One + Copy +
                 Add<Output = W> + Sub<Output = W> +
                 Mul<Output = W> + Div<Output = W> +
                 AddAssign + MulAssign
    {
        if self.elements[0].len() != other.elements.len()
        {
            return Err("cannot solve equations!");
        }
        let mut result_elements = Vec::new();
        for _ in 0..other.elements.len()
        {
            result_elements.push(vec![Default::default()]);
        }
        let mut lhs_elements = self._convert_elements::<V>().elements;
        let mut rhs_elements = other._convert_elements::<V>().elements;
        for k in 0..other.elements.len() - 1
        {
            for i in (k + 1)..other.elements.len()
            {
                let current_coeff = lhs_elements[i][k] / lhs_elements[k][k];
                for j in (k + 1)..other.elements.len()
                {
                    let current_lhs_value = lhs_elements[k][j];
                    lhs_elements[i][j] -= current_coeff * current_lhs_value;
                }
                let current_rhs_value = rhs_elements[k][0];
                rhs_elements[i][0] -= current_coeff * current_rhs_value;
            }
        }
        result_elements[other.elements.len() - 1][0] =
            rhs_elements[other.elements.len() - 1][0] /
            lhs_elements[other.elements.len() - 1][other.elements.len() - 1];
        for i in (0..other.elements.len() - 1).into_iter().rev()
        {
            let mut sum = rhs_elements[i][0];
            for j in (i + 1)..other.elements.len()
            {
                sum -= lhs_elements[i][j] * result_elements[j][0];
            }
            result_elements[i][0] = sum / lhs_elements[i][i];
        }
        Ok(Matrix { elements: result_elements })
    }


    fn inverse<V>(&self) -> Result<Matrix<V>, &str>
        where V: Default + From<T> + One + Copy +
                 Sub + Mul + Div + Add<Output = V> +
                 Sub<Output = V> + Mul<Output = V> +
                 Div<Output = V> + Add + AddAssign +
                 MulAssign + SubAssign + Debug
    {
        let l_u_matrices = self.decompose_to_l_u::<V>()?;
        let lower_matrix = l_u_matrices.get(&DecomposedMatrix::L).unwrap();
        let upper_matrix = l_u_matrices.get(&DecomposedMatrix::U).unwrap();
        let mut inverse_elements = Vec::new();
        for _ in 0..self.elements.len()
        {
            let mut current_row = Vec::new();
            for _ in 0..self.elements[0].len()
            {
                current_row.push(Default::default());
            }
            inverse_elements.push(current_row);
        }
        for k in 0..self.elements[0].len()
        {
            let mut current_unit_elements = Vec::new();
            for _ in 0..self.elements[0].len()
            {
                current_unit_elements.push(vec![Default::default()]);
            }
            current_unit_elements[k][0] = One::one();
            let interim_inverse_column = lower_matrix
                .solve_equations::<V, V>(Matrix { elements: current_unit_elements })
                .unwrap();
            let inverse_column = upper_matrix
                .solve_equations::<V, V>(interim_inverse_column)
                .unwrap();
            for i in 0..inverse_column.elements.len()
            {
                inverse_elements[i][k] = inverse_column.elements[i][0];
            }
        }
        Ok(Matrix { elements: inverse_elements })
    }
}


fn main()
{
    let m_1 = Matrix { elements: vec![vec![1, 2, 3], vec![4, 5, 6]] };
    let m_2 = Matrix { elements: vec![vec![1, 2], vec![3, 4], vec![5, 6]] };
    let m_3 = m_1.multiply(&m_2);
    println!("{:?}", m_3);

    let m_4 = Matrix { elements: vec![vec![1, 2, 3], vec![4, 5, 6]] };
    let m_5 = Matrix { elements: vec![vec![1, 2, 3], vec![4, 5, 6]] };
    let m_6 = m_4.sum(&m_5);
    println!("{:?}", m_6);

    let m_7 = Matrix { elements: vec![vec![3.0, -0.1, -0.2], vec![0.1, 7.0, -0.3], vec![0.3, -0.2, 10.0]] };
    // let m_7 = Matrix { elements: vec![vec![3, -1, -2], vec![1, 7, -3], vec![3, -2, 10]] };
    let decomposed = m_7.decompose_to_l_u::<f64>();
    println!("{:?}", decomposed);

    if let Ok(h_m) = decomposed
    {
        let m_7_upper = h_m.get(&DecomposedMatrix::U).unwrap();
        let m_7_lower = h_m.get(&DecomposedMatrix::L).unwrap();
        let m_8 = m_7_lower.multiply(m_7_upper);
        println!("{:?}", m_8);

        let det_u = m_7_upper.determinant_u();
        println!("{}", det_u);
    }

    let m_9 = Matrix { elements: vec![vec![1, 2, 3]] };
    let m_10 = m_9.transpose();
    println!("{:?}", m_10);

    let m_11 = Matrix { elements: vec![vec![7.85], vec![-19.3], vec![71.4]] };
    if let Ok(m_12) = m_7.solve_equations::<f64, f64>(m_11)
    {
        println!("{:?}", m_12);
    }

    let m_13 = m_7.inverse::<f64>();
    println!("{:?}", m_13);

    println!("{:?}", m_7._convert_elements::<f64>().multiply(&m_13.unwrap()));
}
