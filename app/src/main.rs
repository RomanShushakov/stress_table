mod math;
use math::matrix::{Matrix, DecomposedMatrix};
use math::vector::{Vector, GlobalCoordinateAxis, GlobalCoordinatePlane};
use math::math_aux_structs::Coordinates;


mod fe;
use fe::node::Node;
use fe::elements::truss::Truss2n2ip;
use fe::elements::element::Element;
use fe::solver::Model;


pub const NUMBER_OF_DOF: i32 = 6;


fn main()
{
    let m_1 = Matrix { elements: vec![vec![1, 2, 3], vec![4, 5, 6]] };
    let m_2 = Matrix { elements: vec![vec![1, 2], vec![3, 4], vec![5, 6]] };
    let m_3 = m_1.multiply_by_matrix(&m_2);
    println!("{:?}", m_3);

    let m_4 = Matrix { elements: vec![vec![1, 2, 3], vec![4, 5, 6]] };
    let m_5 = Matrix { elements: vec![vec![1, 2, 3], vec![4, 5, 6]] };
    let m_6 = m_4.sum(&m_5);
    println!("{:?}", m_6);

    let m_7 = Matrix
    {
        elements: vec![vec![3.0, -3.0, 5.0], vec![-3.0, 4.0, 5.0], vec![5.0, 5.0, 5.0]]
    };
    // let m_7 = Matrix
    //     {
    //         elements: vec![vec![3.0, -0.1, -0.2], vec![0.1, 7.0, -0.3], vec![0.3, -0.2, 10.0]]
    //     };
    // let m_7 = Matrix { elements: vec![vec![3, -1, -2], vec![1, 7, -3], vec![3, -2, 10]] };
    let decomposed = m_7.decompose_to_l_u::<f64>();
    println!("{:?}", decomposed);

    if let Ok(h_m) = decomposed
    {
        let m_7_upper = h_m.get(&DecomposedMatrix::U).unwrap();
        let m_7_lower = h_m.get(&DecomposedMatrix::L).unwrap();
        let m_8 = m_7_lower.multiply_by_matrix(m_7_upper);
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
        println!("{:?} m_12", m_12);
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

    let n_1 = Node { number: 1, coordinates: Coordinates { x: 0, y: 0, z: 0 } };
    println!("{:?}", n_1.coordinates);

    let t_x = Matrix
    {
        elements: vec!
            [
                vec![1.0, 0.0, 0.0],
                vec!
                    [
                        0.0,
                        v_1.project_on_coord_plane(GlobalCoordinatePlane::YZ)
                            .cos_coord_axis::<f64>(GlobalCoordinateAxis::Y),
                        -v_1.project_on_coord_plane(GlobalCoordinatePlane::YZ)
                            .sin_coord_axis::<f64>(GlobalCoordinateAxis::Y),
                    ],
                vec!
                    [
                        0.0,
                         -v_1.project_on_coord_plane(GlobalCoordinatePlane::YZ)
                             .sin_coord_axis::<f64>(GlobalCoordinateAxis::Y),
                         v_1.project_on_coord_plane(GlobalCoordinatePlane::YZ)
                            .cos_coord_axis::<f64>(GlobalCoordinateAxis::Y),
                    ]
            ]
    };
    let t_y = Matrix
    {
        elements: vec!
            [
                vec!
                    [
                        v_1.project_on_coord_plane(GlobalCoordinatePlane::XZ)
                            .cos_coord_axis::<f64>(GlobalCoordinateAxis::X),
                        0.0,
                        -v_1.project_on_coord_plane(GlobalCoordinatePlane::XZ)
                            .sin_coord_axis::<f64>(GlobalCoordinateAxis::X)
                    ],
                vec![0.0, 1.0, 0.0],
                vec!
                    [
                        -v_1.project_on_coord_plane(GlobalCoordinatePlane::XZ)
                            .sin_coord_axis::<f64>(GlobalCoordinateAxis::X),
                        0.0,
                        v_1.project_on_coord_plane(GlobalCoordinatePlane::XZ)
                            .cos_coord_axis::<f64>(GlobalCoordinateAxis::X),
                    ]
            ]
    };
    let t_z = Matrix
    {
        elements: vec!
            [
                vec!
                    [
                        v_1.project_on_coord_plane(GlobalCoordinatePlane::XY)
                            .cos_coord_axis::<f64>(GlobalCoordinateAxis::X),
                        -v_1.project_on_coord_plane(GlobalCoordinatePlane::XY)
                            .sin_coord_axis::<f64>(GlobalCoordinateAxis::X),
                        0.0
                    ],
                vec!
                    [
                        -v_1.project_on_coord_plane(GlobalCoordinatePlane::XY)
                            .sin_coord_axis::<f64>(GlobalCoordinateAxis::X),
                        v_1.project_on_coord_plane(GlobalCoordinatePlane::XY)
                            .cos_coord_axis::<f64>(GlobalCoordinateAxis::X),
                        0.0
                    ],
                vec![0.0, 0.0, 1.0],
            ]
    };
    println!("TX Matrix - {:?}", t_x);
    println!("TY Matrix - {:?}", t_y);
    println!("TZ Matrix - {:?}", t_z);
    if let Ok(t) = t_x.multiply_by_matrix(&t_y)
    {
        println!("T Matrix - {:?}", t.multiply_by_matrix(&t_z).unwrap());
    }

    let alpha = -45_f64 * std::f64::consts::PI / 180_f64;
    let t_elements = vec!
        [
            vec![alpha.cos(), alpha.sin(), 0.0, 0.0],
            vec![-alpha.sin(), alpha.cos(), 0.0, 0.0],
            vec![0.0, 0.0, alpha.cos(), alpha.sin()],
            vec![0.0, 0.0, -alpha.sin(), alpha.cos()],
        ];
    let t = Matrix { elements: t_elements };
    let k_elements = vec!
        [
            vec![13.0 / 240.0, 0.0, -13.0 / 240.0, 0.0],
            vec![0.0, 0.0, 0.0, 0.0],
            vec![-13.0 / 240.0, 0.0, 13.0 / 240.0, 0.0],
            vec![0.0, 0.0, 0.0, 0.0],
        ];
    let k_loc = Matrix { elements: k_elements };
    if let Ok(m) = t.transpose().multiply_by_matrix(&k_loc)
    {
        if let Ok(k_glob) = m.multiply_by_matrix(&t)
        {
            println!("{:?}", k_glob);
        }
    }

    let u_glob_elements = vec!
        [
            vec![0.0],
            vec![0.0],
            vec![70.7107 * 480.0 / 26.0],
            vec![70.7107 * 480.0 / 26.0],
        ];
    let u_glob = Matrix { elements: u_glob_elements };
    if let Ok(u_loc) = t.multiply_by_matrix(&u_glob)
    {
        println!("{:?}", u_loc);
    }

    let elements_14 = vec!
        [
            vec![1.0, 0.0],
            vec![0.0, 1.0],
        ];
    let m_14 = Matrix { elements: elements_14 };
    let elements_15 = vec!
        [
            vec![0.0],
            vec![0.0],
        ];
    let m_15 = Matrix { elements: elements_15 };
    if let Ok(result) = m_14.solve_equations::<f64, f64>(m_15)
    {
        println!("{:?}", result.elements[0][0].is_nan());
    }


    let node_1 = Node { number: 1, coordinates: Coordinates { x: 2.0, y: 3.0, z: 0.0 } };
    // let node_2 = Node { number: 2, coordinates: Coordinates { x: 58.57, y: 59.57, z: 0.0 } };
    let node_2 = Node { number: 2, coordinates: Coordinates { x: 82.0, y: 3.0, z: 0.0 } };
    let mut element_1 = Truss2n2ip::create(1, node_1.clone(), node_2.clone(), 1, 1, Some(9));

    println!("{:?}", element_1.compose_rotation_matrix());
    println!("{:?}", element_1.compose_local_stiffness_matrix());
    println!("{:?}", element_1.convert_stiffness_matrix_into_global());

    let force_elements = vec!
        [
            vec![100], vec![0], vec![0], vec![0], vec![0], vec![0],
            vec![0], vec![0], vec![0], vec![0], vec![0], vec![0],
        ];
    let mut force_matrix = Matrix { elements: force_elements };

    if let Ok(mut stiffness_matrix) = element_1.convert_stiffness_matrix_into_global()
    {
        let mut i = 0;
        while i < stiffness_matrix.elements.len()
        {
            if stiffness_matrix.elements[i][i] == 0f64
            {
                stiffness_matrix.elements.remove(i);
                force_matrix.elements.remove(i);
                for j in 0..stiffness_matrix.elements.len()
                {
                    stiffness_matrix.elements[j].remove(i);
                }
                continue;
            }
            i += 1;
        }
        println!("{:?}", stiffness_matrix);
        println!("{:?}", force_matrix);
        let displacements = stiffness_matrix.solve_equations::<f64, i32>(force_matrix);
        println!("{:?}", displacements);
    }

    let k = Matrix
    {
        elements: vec![vec![3.0]]
    };
    let r = Matrix { elements: vec![vec![7.85]] };
    if let Ok(u) = k.solve_equations::<f64, f64>(r)
    {
        println!("{:?} u", u);
    }

    println!("{:?}", element_1.state.displacements);


    let mut nodes = vec![node_2, node_1];
    nodes.sort_unstable_by(|a, b| a.number.partial_cmp(&b.number).unwrap());
    let mut elements: Vec<Box<dyn Element<_, _, _>>> = Vec::new();
    elements.push(Box::new(element_1));
    let model = Model { nodes, elements };
    for element in &model.elements
    {
        println!("{:?}", element.extract_stiffness_submatrices());
    }
    println!("{:?}", model.compose_global_stiffness_matrix());


    // let n_5 = Node { number: 5, coordinates: Coordinates { x: 5.0, y: 0.0, z: 0.0 } };
    // let n_6 = Node { number: 6, coordinates: Coordinates { x: 5.0, y: 10.0, z: 0.0 } };
    // let n_3 = Node { number: 3, coordinates: Coordinates { x: 3.0, y: 12.0, z: 0.0 } };
    // let n_8 = Node { number: 8, coordinates: Coordinates { x: 33.0, y: 7.0, z: 0.0 } };
    //
    // let mut nodes = Vec::new();
    // nodes.push(n_5);
    // nodes.push(n_8);
    // nodes.push(n_3);
    // nodes.push(n_6);
    // nodes.sort_unstable_by(|a, b| a.number.partial_cmp(&b.number).unwrap());
    // println!("{:?}", nodes);
}
