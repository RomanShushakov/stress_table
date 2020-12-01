mod math;
use math::matrix::{Matrix, DecomposedMatrix};
use math::vector::{Vector, GlobalCoordinateAxis, GlobalCoordinatePlane};
use math::math_aux_structs::Coordinates;


mod fe;
use fe::node::Node;
use fe::elements::truss::Truss2n2ip;
use fe::elements::element::Element;
use fe::solver::Model;
use std::rc::Rc;
use std::cell::RefCell;
use crate::fe::fe_aux_structs::{Displacement, Component, Force};
use std::collections::HashMap;


pub const NUMBER_OF_DOF: i32 = 6;


fn main() -> Result<(), String>
{
    let node_1 = Node { number: 1, coordinates: Coordinates { x: 2.0, y: 3.0, z: 0.0 } };
    let node_2 = Node { number: 2, coordinates: Coordinates { x: 58.57, y: 59.57, z: 0.0 } };
    // let node_2 = Node { number: 2, coordinates: Coordinates { x: 82.0, y: 3.0, z: 0.0 } };
    let node_3 = Node { number: 3, coordinates: Coordinates { x: -54.57, y: 59.57, z: 0.0 } };
    let node_4 = Node { number: 4, coordinates: Coordinates { x: 2.0, y: 59.57, z: 56.57 } };
    let mut nodes = vec![node_2.to_owned(), node_3.to_owned(), node_1.to_owned(), node_4.to_owned()];
    // let mut nodes = vec![node_1.to_owned(), node_2.to_owned()];
    nodes.sort_unstable_by(|a, b| a.number.partial_cmp(&b.number).unwrap());

    let element_1 = Truss2n2ip::create
        (
            1, node_1.to_owned(), node_2.to_owned(),
            1, 1, Some(9)
        );
    let element_2 = Truss2n2ip::create
        (
            2, node_1.to_owned(), node_3.to_owned(),
            1, 1, Some(9)
        );
    let element_3 = Truss2n2ip::create
        (
            3, node_4.to_owned(), node_1.to_owned(),
            1, 1, Some(9)
        );
    let mut elements: Vec<Rc<RefCell<dyn Element<_, _, _>>>> = Vec::new();
    elements.push(Rc::new(RefCell::new(element_1)));
    elements.push(Rc::new(RefCell::new(element_2)));
    elements.push(Rc::new(RefCell::new(element_3)));

    let mut applied_displacements = HashMap::new();
    applied_displacements.insert(Displacement { component: Component::W, node_number: 4 }, 0);
    applied_displacements.insert(Displacement { component: Component::V, node_number: 4 }, 0);
    applied_displacements.insert(Displacement { component: Component::U, node_number: 2 }, 0);
    applied_displacements.insert(Displacement { component: Component::V, node_number: 2 }, 0);
    applied_displacements.insert(Displacement { component: Component::U, node_number: 3 }, 0);
    applied_displacements.insert(Displacement { component: Component::V, node_number: 3 }, 0);

    let mut applied_forces = HashMap::new();
    applied_forces.insert(Force { component: Component::V, node_number: 1 }, -100);
    applied_forces.insert(Force { component: Component::U, node_number: 1 }, 112);
    applied_forces.insert(Force { component: Component::W, node_number: 1 }, 112);

    let mut model = Model::create(nodes, elements, applied_displacements, applied_forces);

    model.compose_global_stiffness_matrix()?;
    if let Some(ref state) = model.state
    {
        println!("{:?}", state.displacements_indexes);
        println!("{:?}", state.forces_indexes);
        println!("{:?}", state.stiffness_matrix);
    }
    model.analyze()?;



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

    // let k_elem = vec![vec![0.05416527108258273, 0.0], vec![0.0, 0.05416527108258273]];
    // // let k_elem = vec![vec![0.05416527108258273, -0.05416527108258273], vec![-0.05416527108258273, 0.05416527108258273]];
    // let k = Matrix { elements: k_elem };
    //
    // let r_elem = vec![vec![100], vec![-100]];
    // let r = Matrix { elements: r_elem };
    //
    // if let Ok(u) = k.solve_equations::<f64, i32>(r)
    // {
    //     println!("{:?}", u);
    // }


    // let elements = vec!
    //     [
    //         vec![-0.86, 0.5, 0.0, 0.0],
    //         vec![-0.5, -0.86, 0.0, 0.0],
    //         vec![0.0, 0.0, -0.86, 0.5],
    //         vec![0.0, 0.0, -0.5, -0.86],
    //     ];
    // let m = Matrix { elements };
    // println!("{:?}", m.transpose().multiply_by_matrix(&m));
    //
    // let elements = vec!
    //     [
    //         vec![0.86, -0.5, 0.0, 0.0],
    //         vec![0.5, 0.86, 0.0, 0.0],
    //         vec![0.0, 0.0, 0.86, -0.5],
    //         vec![0.0, 0.0, 0.5, 0.86],
    //     ];
    // let m = Matrix { elements };
    // println!("{:?}", m.transpose().multiply_by_matrix(&m));


    Ok(())
}
