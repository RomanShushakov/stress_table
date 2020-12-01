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
    let node_1 = Node { number: 1, coordinates: Coordinates { x: 0.0, y: 0.0, z: 0.0 } };
    let node_2 = Node { number: 2, coordinates: Coordinates { x: 80.0, y: 0.0, z: 0.0 } };
    // let node_2 = Node { number: 2, coordinates: Coordinates { x: 56.57, y: 56.57, z: 0.0 } };
    // let node_3 = Node { number: 3, coordinates: Coordinates { x: -56.57, y: 56.57, z: 0.0 } };
    // let node_4 = Node { number: 4, coordinates: Coordinates { x: 0.0, y: 56.57, z: 56.57 } };
    // let node_5 = Node { number: 5, coordinates: Coordinates { x: 0.0, y: 56.57, z: -56.57 } };
    // let mut nodes = vec![node_2.to_owned(), node_3.to_owned(), node_1.to_owned(), node_4.to_owned(), node_5.to_owned()];
    let mut nodes = vec![node_2.to_owned(), node_1.to_owned()];
    nodes.sort_unstable_by(|a, b| a.number.partial_cmp(&b.number).unwrap());

    let element_1 = Truss2n2ip::create
        (
            1, node_1.to_owned(), node_2.to_owned(),
            1, 1, Some(9)
        );
    // let element_2 = Truss2n2ip::create
    //     (
    //         2, node_1.to_owned(), node_3.to_owned(),
    //         1, 1, Some(9)
    //     );
    // let element_3 = Truss2n2ip::create
    //     (
    //         3, node_4.to_owned(), node_1.to_owned(),
    //         1, 1, Some(9)
    //     );
    // let element_4 = Truss2n2ip::create
    //     (
    //         4, node_1.to_owned(), node_5.to_owned(),
    //         1, 1, Some(9)
    //     );
    let mut elements: Vec<Rc<RefCell<dyn Element<_, _, _>>>> = Vec::new();
    elements.push(Rc::new(RefCell::new(element_1)));
    // elements.push(Rc::new(RefCell::new(element_2)));
    // elements.push(Rc::new(RefCell::new(element_3)));
    // elements.push(Rc::new(RefCell::new(element_4)));

    let mut applied_displacements = HashMap::new();
    applied_displacements.insert(Displacement { component: Component::U, node_number: 2 }, 0);
    // applied_displacements.insert(Displacement { component: Component::V, node_number: 2 }, 0);
    // applied_displacements.insert(Displacement { component: Component::U, node_number: 3 }, 0);
    // applied_displacements.insert(Displacement { component: Component::V, node_number: 3 }, 0);
    // applied_displacements.insert(Displacement { component: Component::V, node_number: 4 }, 0);
    // applied_displacements.insert(Displacement { component: Component::W, node_number: 4 }, 0);
    // applied_displacements.insert(Displacement { component: Component::V, node_number: 5 }, 0);
    // applied_displacements.insert(Displacement { component: Component::W, node_number: 5 }, 0);

    let mut applied_forces = HashMap::new();
    applied_forces.insert(Force { component: Component::U, node_number: 1 }, 100);
    // applied_forces.insert(Force { component: Component::V, node_number: 1 }, 100);
    // applied_forces.insert(Force { component: Component::W, node_number: 1 }, 100);

    let mut model = Model::create(nodes, elements, applied_displacements, applied_forces);

    model.compose_global_stiffness_matrix()?;
    if let Some(ref state) = model.state
    {
        println!("{:?}", state.displacements_indexes);
        println!("{:?}", state.forces_indexes);
        println!("{:?}", state.stiffness_matrix);
    }
    model.analyze()?;
    if let Some(analysis_result) = model.analysis_result
    {
        println!("{:?}", analysis_result.reactions);
        println!("{:?}", analysis_result.displacements);
    }

    Ok(())
}
