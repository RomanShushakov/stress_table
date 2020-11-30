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
use crate::fe::fe_aux_structs::{Displacement, DisplacementComponent, Force, ForceComponent};
use std::collections::HashMap;


pub const NUMBER_OF_DOF: i32 = 6;


fn main()
{
    let node_1 = Node { number: 1, coordinates: Coordinates { x: 2.0, y: 3.0, z: 0.0 } };
    // let node_2 = Node { number: 2, coordinates: Coordinates { x: 58.57, y: 59.57, z: 0.0 } };
    let node_2 = Node { number: 2, coordinates: Coordinates { x: 82.0, y: 3.0, z: 0.0 } };
    // let node_5 = Node { number: 5, coordinates: Coordinates { x: -54.57, y: 59.57, z: 0.0 } };
    let mut nodes = vec![node_2.to_owned(), node_1.to_owned()];
    nodes.sort_unstable_by(|a, b| a.number.partial_cmp(&b.number).unwrap());

    let element_1 = Truss2n2ip::create(1, node_1.to_owned(), node_2.to_owned(), 1, 1, Some(9));
    // let element_3 = Truss2n2ip::create(3, node_1.to_owned(), node_5.to_owned(), 1, 1, Some(9));
    let mut elements: Vec<Rc<RefCell<dyn Element<_, _, _>>>> = Vec::new();
    elements.push(Rc::new(RefCell::new(element_1)));
    // elements.push(Rc::new(RefCell::new(element_3)));

    let mut displacement_1 = HashMap::new();
    displacement_1.insert(Displacement { component: DisplacementComponent::U, node_number: 1 }, 0);
    let displacements = vec![displacement_1];

    let mut force_1 = HashMap::new();
    force_1.insert(Force { component: ForceComponent::RU, node_number: 2 }, 100);
    let forces = vec![force_1];


    let mut model = Model::create(nodes, elements, displacements, forces);
    if let Ok(_) = model.compose_global_stiffness_matrix()
    {
        let state = model.state.unwrap();
        println!("{:?}", state.displacements_indexes);
        println!("{:?}", state.forces_indexes);
        println!("{:?}", state.stiffness_matrix);
    }




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
