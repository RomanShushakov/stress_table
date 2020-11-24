mod math;
use math::matrix::{Matrix, DecomposedMatrix};
use math::vector::{Vector, GlobalCoordinateAxis, GlobalCoordinatePlane};
use math::aux_structs::Coordinates;


mod fe;
use fe::node::Node;


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

    // println!("{:?}", m_7._convert_elements::<f64>().multiply(&m_13.unwrap()));

    let v_1 = Vector
    {
        start_coordinates: Coordinates { x: 0, y: 0, z: 0 },
        end_coordinates: Coordinates { x: 3, y: 3, z: 0 },
    };
    println!("{:?}", v_1.cos_coord_axis::<f64>(GlobalCoordinateAxis::X));
    println!("{:?}", v_1.sin_coord_axis::<f64>(GlobalCoordinateAxis::X));

    let n_1 = Node { number: 1, coordinates: Coordinates { x: 0.0, y: 0.0, z: 0.0 } };
    println!("{:?}", n_1.coordinates);

    let t_x = Matrix
    {
        elements: vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0,
                v_1.project_on_coord_plane(GlobalCoordinatePlane::YZ)
                    .cos_coord_axis::<f64>(GlobalCoordinateAxis::Y),
                -v_1.project_on_coord_plane(GlobalCoordinatePlane::YZ)
                    .sin_coord_axis::<f64>(GlobalCoordinateAxis::Y),],
            vec![0.0,
                 -v_1.project_on_coord_plane(GlobalCoordinatePlane::YZ)
                     .sin_coord_axis::<f64>(GlobalCoordinateAxis::Y),
                 v_1.project_on_coord_plane(GlobalCoordinatePlane::YZ)
                     .cos_coord_axis::<f64>(GlobalCoordinateAxis::Y),]
        ]
    };
    let t_y = Matrix
    {
        elements: vec![
            vec![v_1.project_on_coord_plane(GlobalCoordinatePlane::XZ)
                     .cos_coord_axis::<f64>(GlobalCoordinateAxis::X),
                 0.0,
                 -v_1.project_on_coord_plane(GlobalCoordinatePlane::XZ)
                     .sin_coord_axis::<f64>(GlobalCoordinateAxis::X)],
            vec![0.0, 1.0, 0.0],
            vec![-v_1.project_on_coord_plane(GlobalCoordinatePlane::XZ)
                    .sin_coord_axis::<f64>(GlobalCoordinateAxis::X),
                 0.0,
                 v_1.project_on_coord_plane(GlobalCoordinatePlane::XZ)
                     .cos_coord_axis::<f64>(GlobalCoordinateAxis::X),]
        ]
    };
    let t_z = Matrix
    {
        elements: vec![
            vec![v_1.project_on_coord_plane(GlobalCoordinatePlane::XY)
                     .cos_coord_axis::<f64>(GlobalCoordinateAxis::X),
                 -v_1.project_on_coord_plane(GlobalCoordinatePlane::XY)
                     .sin_coord_axis::<f64>(GlobalCoordinateAxis::X),
                 0.0],
            vec![-v_1.project_on_coord_plane(GlobalCoordinatePlane::XY)
                    .sin_coord_axis::<f64>(GlobalCoordinateAxis::X),
                 v_1.project_on_coord_plane(GlobalCoordinatePlane::XY)
                     .cos_coord_axis::<f64>(GlobalCoordinateAxis::X),
                 0.0],
            vec![0.0, 0.0, 1.0],
        ]
    };
    println!("TX Matrix - {:?}", t_x);
    println!("TY Matrix - {:?}", t_y);
    println!("TZ Matrix - {:?}", t_z);
    if let Ok(t) = t_x.multiply(&t_y)
    {
        println!("T Matrix - {:?}", t.multiply(&t_z).unwrap());
    }

}
