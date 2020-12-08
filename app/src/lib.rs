#![recursion_limit="1024"]

mod math;
use math::math_aux_structs::Coordinates;


mod fe;
use fe::node::FeNode;
use fe::elements::truss::Truss2n2ip;
use fe::elements::element::FElement;
use fe::solver::FeModel;
use std::rc::Rc;
use std::cell::RefCell;
use crate::fe::fe_aux_structs::{Displacement, AxisComponent};
use std::collections::HashMap;


pub const NUMBER_OF_DOF: i32 = 6;


fn result_extract() -> Result<f64, String>
{
    let node_3 = FeNode { number: 3, coordinates: Coordinates { x: 0.0, y: 0.0, z: 0.0 } };
    let node_4 = FeNode { number: 4, coordinates: Coordinates { x: 0.0, y: 3.0, z: 0.0 } };
    let node_2 = FeNode { number: 2, coordinates: Coordinates { x: 4.0, y: 3.0, z: 0.0 } };
    let node_1 = FeNode { number: 1, coordinates: Coordinates { x: 4.0, y: 0.0, z: 0.0 } };
    let mut nodes = vec![node_2.to_owned(), node_1.to_owned(), node_3.to_owned(), node_4.to_owned()];
    nodes.sort_unstable_by(|a, b| a.number.partial_cmp(&b.number).unwrap());

    let element_1 = Truss2n2ip::create
        (
            1, node_2.to_owned(), node_1.to_owned(),
            128000000.0, 0.0625, None
        );
    let element_2 = Truss2n2ip::create
        (
            2, node_2.to_owned(), node_3.to_owned(),
            128000000.0, 0.0625, None
        );
    let element_3 = Truss2n2ip::create
        (
            3, node_2.to_owned(), node_4.to_owned(),
            128000000.0, 0.0625, None
        );

    let mut elements: Vec<Rc<RefCell<dyn FElement<_, _, _>>>> = Vec::new();
    elements.push(Rc::new(RefCell::new(element_1)));
    elements.push(Rc::new(RefCell::new(element_2)));
    elements.push(Rc::new(RefCell::new(element_3)));

    let mut applied_displacements = HashMap::new();
    applied_displacements.insert(Displacement { component: AxisComponent::U, node_number: 3 }, 0.0);
    applied_displacements.insert(Displacement { component: AxisComponent::V, node_number: 3 }, 0.0);
    applied_displacements.insert(Displacement { component: AxisComponent::U, node_number: 4 }, 0.0);
    // applied_displacements.insert(Displacement { component: Component::V, node_number: 4 }, 0.0);
    applied_displacements.insert(Displacement { component: AxisComponent::V, node_number: 1 }, -0.025);

    // let mut applied_forces = HashMap::new();
    // applied_forces.insert(Force { component: Component::V, node_number: 1 }, -100.0);
    // applied_forces.insert(Force { component: Component::V, node_number: 1 }, 100);
    // applied_forces.insert(Force { component: Component::W, node_number: 1 }, 100);

    let mut model = FeModel::create(nodes, elements, applied_displacements, None);

    model.compose_global_stiffness_matrix()?;
    // if let Some(ref state) = model.state
    // {
    //     println!("{:?}", state.displacements_indexes);
    //     println!("{:?}", state.forces_indexes);
    //     println!("{:?}", state.stiffness_matrix);
    // }
    model.analyze()?;

    let mut max_stress = 0f64;

    if let Some(ref analysis_result) = model.analysis_result
    {
        println!("Reactions: {:?}", analysis_result.reactions);
        println!("Displacements: {:?}", analysis_result.displacements);
    }
    for element in model.elements
    {
        let global_displacements =
            &model.analysis_result.as_ref().unwrap().displacements;
        let strains_and_stresses =
            element
                .borrow_mut()
                .calculate_strains_and_stresses(global_displacements)?;
        for (k, v) in strains_and_stresses
        {
            for stress_strain in v
            {
                if stress_strain.stress.value > max_stress
                {
                    max_stress = stress_strain.stress.value;
                }
            }
            // println!("For element: {:?}, strains and stresses are: {:?}", k, v);
        }
    }
    Ok(max_stress)
}


use wasm_bindgen::prelude::*;
use yew::prelude::*;
use std::f64;
use wasm_bindgen::JsCast;
use web_sys::Node;
use yew::virtual_dom::VNode;
use web_sys::
    {
        CanvasRenderingContext2d, HtmlSelectElement, HtmlOptionElement, HtmlCanvasElement,
        HtmlOptionsCollection, DomTokenList,
    };
use yew::services::resize::{WindowDimensions, ResizeTask};
use yew::services::ResizeService;

mod components;
use components::NodesMenu;


const CANVAS_ID: &str = "canvas";
// const NODES_MENU_CONTAINER_ID: &str = "nodes_menu_container";
// const NODES_MENU_CONTAINER: &str = "nodes_menu_container";
// const HIDDEN: &str = "hidden";
// const NODE_SELECT_ID: &str = "node_select";


struct State
{
    canvas_width: u32,
    canvas_height: u32,
    nodes: Vec<FeNode<u16, f64>>,
    // selected_node: FeNode<u16, f64>,
    max_stress: Option<f64>,
}


struct Model
{
    link: ComponentLink<Self>,
    state: State,
    resize_task: Option<ResizeTask>,
    resize_service: ResizeService,
}


enum Msg
{
    ExtractWindowDimensions(WindowDimensions),
    // ShowHideNodesMenu,
    // SelectNode(ChangeData),
    // UpdateEditXCoord(String),
    // UpdateEditYCoord(String),
    // ApplyNodeDataChange,
    // RemoveNode,
    ShowResult,
}


impl Model
{
    fn follow_window_dimensions(&mut self)
    {
        let callback: Callback<WindowDimensions> = self.link
            .callback(|dimensions| Msg::ExtractWindowDimensions(dimensions));
        let task = ResizeService::register(&mut self.resize_service, callback);
        self.resize_task = Some(task);
    }


    fn extract_window_dimensions(&mut self, dimensions: WindowDimensions)
    {
        self.state.canvas_width = (dimensions.width as f32 * 0.8) as u32;
        self.state.canvas_height = (dimensions.height as f32 * 0.8) as u32;
    }


    fn draw_canvas(&self) -> Html
    {

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let element = document.create_element("canvas").unwrap();
        element.set_id(CANVAS_ID);
        let canvas = element.dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        canvas.set_width(self.state.canvas_width);
        canvas.set_height(self.state.canvas_height);
        let base_dimension =
            {
                if self.state.canvas_width < self.state.canvas_height
                {
                    self.state.canvas_width
                }
                else
                {
                    self.state.canvas_height
                }
            };
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let x_origin = base_dimension as f64 / 20f64;
        let y_origin = base_dimension as f64 - base_dimension as f64 / 20f64;
        let axis_line_length = base_dimension as f64 / 7f64;
        let axis_line_width = axis_line_length / 50f64;

        context.begin_path();
        context.move_to(x_origin, y_origin);
        context.set_line_width(axis_line_width);
        context.set_stroke_style(&"red".into());
        context.line_to(x_origin + axis_line_length - axis_line_length / 7f64, y_origin);
        context.move_to(x_origin + axis_line_length, y_origin);
        context.line_to(x_origin + axis_line_length - axis_line_length / 7f64, y_origin - axis_line_length / 25f64);
        context.line_to(x_origin + axis_line_length - axis_line_length / 7f64, y_origin + axis_line_length / 25f64);
        context.line_to(x_origin + axis_line_length, y_origin);
        context.set_fill_style(&"red".into());
        context.fill();
        context.set_font(&format!("{}px Serif", axis_line_length as i32 / 6));
        context.fill_text("X", x_origin + axis_line_length + axis_line_length / 10f64, y_origin + axis_line_length / 7f64).unwrap();
        context.stroke();

        context.begin_path();
        context.move_to(x_origin, y_origin);
        context.set_stroke_style(&"green".into());
        context.line_to(x_origin, y_origin - axis_line_length + axis_line_length / 7f64);
        context.move_to(x_origin, y_origin - axis_line_length);
        context.line_to(x_origin - axis_line_length / 25f64, y_origin - axis_line_length + axis_line_length / 7f64);
        context.line_to(x_origin + axis_line_length / 25f64, y_origin - axis_line_length + axis_line_length / 7f64);
        context.line_to(x_origin, y_origin - axis_line_length);
        context.set_fill_style(&"green".into());
        context.fill();
        context.set_font(&format!("{}px Serif", axis_line_length as i32 / 6));
        context.fill_text("Y", x_origin - axis_line_length / 7f64, y_origin - axis_line_length - axis_line_length / 10f64).unwrap();
        context.stroke();

        let node = Node::from(canvas);
        let vnode = VNode::VRef(node);
        vnode
    }


    // fn show_hide_nodes_menu(&self)
    // {
    //     let window = web_sys::window().unwrap();
    //     let document = window.document().unwrap();
    //     let element = document.get_element_by_id(NODES_MENU_CONTAINER_ID).unwrap();
    //     let class_list: DomTokenList = element.class_list();
    //     if class_list.contains(HIDDEN)
    //     {
    //         element.set_class_name(NODES_MENU_CONTAINER);
    //     }
    //     else
    //     {
    //         element.set_class_name(&(NODES_MENU_CONTAINER.to_owned() + " " + HIDDEN));
    //     }
    // }


    // fn update_node_menu(&mut self)
    // {
    //     let window = web_sys::window().unwrap();
    //     let document = window.document().unwrap();
    //     let element = document.get_element_by_id(NODE_SELECT_ID).unwrap();
    //     let select = element.dyn_into::<HtmlSelectElement>()
    //         .map_err(|_| ())
    //         .unwrap();
    //     let options: HtmlOptionsCollection = select.options();
    //     options.set_length(self.state.nodes.len() as u32 + 1);
    //     let number =
    //         {
    //             let mut n = 0;
    //             for (i, node) in self.state.nodes.iter().enumerate()
    //             {
    //                 if let Ok(option) = HtmlOptionElement::new()
    //                 {
    //                     option.set_value(&node.number.to_string());
    //                     option.set_text(&node.number.to_string());
    //                     options.set(i as u32, Some(&option)).unwrap();
    //                 }
    //                 if node.number > n
    //                 {
    //                     n = node.number;
    //                 }
    //             }
    //             n + 1
    //         };
    //     let (x, y, z) = (0.0, 0.0, 0.0);
    //     self.state.selected_node = FeNode { number, coordinates: Coordinates { x, y, z } };
    //     if let Ok(option) = HtmlOptionElement::new()
    //     {
    //         option.set_value(&number.to_string());
    //         option.set_text(&format!("{} New", number));
    //         options.set(self.state.nodes.len() as u32, Some(&option)).unwrap();
    //     }
    //     options.set_selected_index(self.state.nodes.len() as i32).unwrap();
    // }
}


impl Component for Model
{
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        let (width, height) =
            {
                let mut width = 320u32;
                let mut height = 240u32;
                let window = web_sys::window().unwrap();
                if let Ok(w) = window.inner_width()
                {
                    if let Some(w) = w.as_f64()
                    {
                        width = (w * 0.8) as u32;
                    }
                }
                if let Ok(h) = window.inner_height()
                {
                    if let Some(h) = h.as_f64()
                    {
                        height = (h * 0.8) as u32;
                    }
                }
                (width, height)
            };
        // let selected_node = FeNode { number: 1, coordinates: Coordinates { x: 0.0, y: 0.0, z: 0.0 } };
        Self
        {
            link,
            state: State
                {
                    canvas_width: width, canvas_height: height, max_stress: None,
                    nodes: Vec::new(), // selected_node
                },
            resize_task: None, resize_service: ResizeService::new(),
        }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::ExtractWindowDimensions(window_dimensions) =>
                self.extract_window_dimensions(window_dimensions),
            // Msg::ShowHideNodesMenu => self.show_hide_nodes_menu(),
            // Msg::SelectNode(data) =>
            //     {
            //         match data
            //         {
            //             ChangeData::Select(select_node) =>
            //                 {
            //                     if let Some(position) = self.state.nodes
            //                             .iter()
            //                             .position(|node| node.number.to_string() == select_node.value())
            //                     {
            //                         self.state.selected_node = self.state.nodes[position].to_owned();
            //                     }
            //                     else
            //                     {
            //                         let number = select_node.value().parse::<u16>().unwrap();
            //                         let (x, y, z) = (0.0, 0.0, 0.0);
            //                         self.state.selected_node = FeNode { number, coordinates: Coordinates { x, y, z } };
            //                     }
            //                 },
            //             _ => (),
            //         }
            //     },
            // Msg::UpdateEditXCoord(e) =>
            //     {
            //         if let Ok(x) = e.parse::<f64>()
            //         {
            //             self.state.selected_node.coordinates.x = x;
            //         }
            //     },
            // Msg::UpdateEditYCoord(e) =>
            //     {
            //         if let Ok(y) = e.parse::<f64>()
            //         {
            //             self.state.selected_node.coordinates.y = y;
            //         }
            //     },
            // Msg::ApplyNodeDataChange =>
            //     {
            //         if let Some(position) = self.state.nodes
            //             .iter()
            //             .position(|node| node.number == self.state.selected_node.number)
            //         {
            //             self.state.nodes[position] = self.state.selected_node.to_owned();
            //         }
            //         else
            //         {
            //             self.state.nodes.push(self.state.selected_node.to_owned());
            //         }
            //         self.update_node_menu();
            //     },
            // Msg::RemoveNode =>
            //     {
            //         if let Some(position) =
            //         self.state.nodes
            //             .iter()
            //             .position(|node| node.number == self.state.selected_node.number)
            //         {
            //             self.state.nodes.remove(position);
            //         }
            //         self.update_node_menu();
            //     },
            Msg::ShowResult =>
                {
                    if let Ok(stress) = result_extract()
                    {
                        self.state.max_stress = Some(stress);
                    }
                },
        }
        true
    }


    fn change(&mut self, _props: Self::Properties) -> ShouldRender
    {
        false
    }


    fn view(&self) -> Html
    {
        html! {
            <div class="container">
                <div class="preprocessor">
                    <div class="buttons">
                        <NodesMenu nodes=self.state.nodes.clone() />
                        // <button
                        //     class="button" onclick=self.link.callback(|_| Msg::ShowHideNodesMenu)
                        // >
                        //     { "Nodes" }
                        // </button>
                        // <div id = { NODES_MENU_CONTAINER_ID } class={ NODES_MENU_CONTAINER.to_owned() + " " + HIDDEN }>
                        //     <div>
                        //         <ul class="nodes_menu">
                        //             <li>
                        //                 {
                        //                     html!
                        //                     {
                        //                         <select
                        //                             id={ NODE_SELECT_ID }
                        //                             onchange=self.link.callback(|data: ChangeData| Msg::SelectNode(data))
                        //                         >
                        //                             <option value={ self.state.selected_node.number }>
                        //                                 { format!("{} New", self.state.selected_node.number) }
                        //                             </option>
                        //                         </select>
                        //                     }
                        //                 }
                        //             </li>
                        //             {
                        //                 html!
                        //                 {
                        //                     <>
                        //                         <li>
                        //                             <p>{ "x coordinate" }</p>
                        //                             <input
                        //                                 value={ self.state.selected_node.coordinates.x }
                        //                                 type="number"
                        //                                 oninput=self.link.callback(|d: InputData| Msg::UpdateEditXCoord(d.value))
                        //                             />
                        //                         </li>
                        //                         <li>
                        //                             <p>{ "y coordinate" }</p>
                        //                             <input
                        //                                 value={ self.state.selected_node.coordinates.y }
                        //                                 type="number"
                        //                                 oninput=self.link.callback(|d: InputData| Msg::UpdateEditYCoord(d.value))
                        //                             />
                        //                         </li>
                        //                     </>
                        //                 }
                        //
                        //             }
                        //         </ul>
                        //     </div>
                        //     <div>
                        //         <button
                        //             class="menu_button"
                        //             onclick=self.link.callback(|_| Msg::ApplyNodeDataChange)
                        //         >
                        //             { "Apply" }
                        //         </button>
                        //         <button
                        //             class="menu_button"
                        //             onclick=self.link.callback(|_| Msg::RemoveNode)
                        //         >
                        //             { "Remove" }
                        //         </button>
                        //     </div>
                        // </div>
                        <button class="button">{ "Elements" }</button>
                        <button class="button">{ "Forces" }</button>
                        <button class="button">{ "Displacements" }</button>
                        <button class="button" onclick=self.link.callback(|_| Msg::ShowResult)>{ "Analyze" }</button>
                    </div>
                    <div class="canvas">
                        { self.draw_canvas() }
                    </div>
                </div>
                {
                    if let Some(max_stress) = self.state.max_stress
                    {
                        html!
                        {
                            <p>{ max_stress }</p>
                        }
                    }
                    else
                    {
                        html! {}
                    }
                }
            </div>
        }
    }


    fn rendered(&mut self, first_render: bool)
    {
        if first_render
        {
            self.follow_window_dimensions();
        }
    }
}


#[wasm_bindgen(start)]
pub fn run_app()
{
    App::<Model>::new().mount_to_body();
}
