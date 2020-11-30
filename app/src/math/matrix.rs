use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign};
use crate::math::math_aux_traits::One;


#[derive(Eq, PartialEq, Hash, Debug)]
pub enum DecomposedMatrix
{
    L,
    U
}


#[derive(Debug, Clone)]
pub struct Matrix<T>
{
    pub elements: Vec<Vec<T>>
}


impl<T> Matrix<T>
    where T: One + Copy + Default +
             Add<Output = T> + Mul<Output = T> +
             AddAssign + MulAssign
{
    pub fn multiply_by_number(&self, number: T) -> Matrix<T>
    {
        let mut elements = Vec::new();
        for i in 0..self.elements.len()
        {
            let mut current_row: Vec<T> = Vec::new();
            for j in 0..self.elements[0].len()
            {
                let current_element = self.elements[i][j] * number;
                current_row.push(current_element);
            }
            elements.push(current_row);
        }
        Matrix { elements }
    }


    pub fn multiply_by_matrix(&self, other: &Matrix<T>) -> Result<Matrix<T>, &str>
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


    pub fn sum(&self, other: &Matrix<T>) -> Result<Matrix<T>, &str>
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


    pub fn decompose_to_l_u<V>(&self) -> Result<HashMap<DecomposedMatrix, Matrix<V>>, &str>
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


    pub fn determinant_u(&self) -> T
    {
        let mut determinant = One::one();
        for i in 0..self.elements.len()
        {
            determinant *= self.elements[i][i]
        }
        determinant
    }


    pub fn transpose(&self) -> Matrix<T>
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


    pub fn solve_equations<V, W>(&self, other: Matrix<W>) -> Result<Matrix<V>, &str>
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


    pub fn inverse<V>(&self) -> Result<Matrix<V>, &str>
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
