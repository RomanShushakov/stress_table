#![recursion_limit="16384"]
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use std::f64;
use yew::services::resize::{WindowDimensions, ResizeTask, ResizeService};
use yew_router::prelude::*;

mod fem;
use crate::fem::{FEModel, FEData, Displacements};
use crate::fem::{GlobalDOFParameter};
use crate::fem::GLOBAL_DOF;
mod extended_matrix;

mod components;

mod auxiliary;
use auxiliary::{View, FEDrawnNodeData, FEDrawnBCData};
use crate::auxiliary::FEDrawnElementData;

mod route;
use route::AppRoute;

mod pages;
use pages::{Preprocessor, Postprocessor};
use crate::fem::global_analysis::fe_global_analysis_result::Reactions;
use crate::fem::element_analysis::fe_element_analysis_result::ElementsAnalysisResult;


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub type ElementsNumbers = u16;
pub type ElementsValues = f64;
pub type GLElementsNumbers = u16;
pub type GLElementsValues = f32;
pub type UIDNumbers = u32;

pub const TOLERANCE: ElementsValues = 1e-6;

const MAIN_CLASS: &str = "main";
const MAIN_CONTAINER_CLASS: &str = "main_container";


struct State
{
    view: Option<View>,
    canvas_width: u32,
    canvas_height: u32,
    is_preprocessor_active: bool,
    fem: FEModel<ElementsNumbers, ElementsValues>,
    analysis_message: Option<String>,
    global_displacements: Rc<Option<Displacements<ElementsNumbers, ElementsValues>>>,
    reactions: Rc<Option<Reactions<ElementsNumbers, ElementsValues>>>,
    elements_analysis_result: Rc<Option<ElementsAnalysisResult<ElementsNumbers, ElementsValues>>>,
}


struct Model
{
    link: ComponentLink<Self>,
    state: State,
    resize_task: Option<ResizeTask>,
}


enum Msg
{
    ExtractWindowDimensions(WindowDimensions),
    ChangeView(View),
    DiscardView,
    AddNode(FEDrawnNodeData),
    UpdateNode(FEDrawnNodeData),
    DeleteNode(ElementsNumbers),
    AddElement(FEDrawnElementData),
    UpdateElement(FEDrawnElementData),
    DeleteElement(ElementsNumbers),
    AddBC(FEDrawnBCData),
    UpdateBC(FEDrawnBCData),
    DeleteBC(FEDrawnBCData),
    AddAnalysisErrorMessage(String),
    ResetAnalysisMessage,
    Submit,
    EditFEM,
}


impl Model
{
    fn follow_window_dimensions(&mut self)
    {
        let mut resize_service = ResizeService::new();
        let callback: Callback<WindowDimensions> = self.link
            .callback(|dimensions| Msg::ExtractWindowDimensions(dimensions));
        let task = ResizeService::register(&mut resize_service, callback);
        self.resize_task = Some(task);
    }


    fn extract_window_dimensions(&mut self, dimensions: WindowDimensions)
    {
        self.state.canvas_width = (dimensions.width as f32 * 0.8) as u32;
        self.state.canvas_height = (dimensions.height as f32 * 0.8) as u32;
    }


    fn submit(&mut self) -> Result<(), String>
    {
        let global_analysis_result = self.state.fem.global_analysis()?;
        let global_displacements =
            global_analysis_result.extract_displacements();
        let reactions = global_analysis_result.extract_reactions();
        let elements_analysis_result =
            self.state.fem.elements_analysis(&global_displacements)?;
        self.state.global_displacements = Rc::new(Some(global_displacements));
        self.state.reactions = Rc::new(Some(reactions));
        self.state.elements_analysis_result = Rc::new(Some(elements_analysis_result));
        let analysis_success_message = "The analysis was successfully completed!".to_string();
        self.state.analysis_message = Some(analysis_success_message);
        self.state.is_preprocessor_active = false;
        Ok(())
    }
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
        let fem = FEModel::create();
        Self
        {
            link,
            state: State
                {
                    view: None,
                    canvas_width: width,
                    canvas_height: height,
                    is_preprocessor_active: true,
                    fem,
                    analysis_message: None,
                    global_displacements: Rc::new(None),
                    reactions: Rc::new(None),
                    elements_analysis_result: Rc::new(None),
                },
            resize_task: None,
        }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::ExtractWindowDimensions(window_dimensions) =>
                self.extract_window_dimensions(window_dimensions),
            Msg::ChangeView(view) => self.state.view = Some(view),
            Msg::DiscardView => self.state.view = None,
            Msg::AddNode(data) =>
                {
                    match self.state.fem.add_node(data.number, data.x, data.y, data.z)
                    {
                        Err(msg) => self.state.analysis_message = Some(msg),
                        _ => (),
                    }
                },
            Msg::UpdateNode(data) => match self.state.fem
                    .update_node(data.number, data.x, data.y, data.z)
                {
                    Err(msg) => self.state.analysis_message = Some(msg),
                    _ => (),
                },
            Msg::DeleteNode(number) => match self.state.fem.delete_node(number)
                {
                    Err(msg) => self.state.analysis_message = Some(msg),
                    _ => (),
                },
            Msg::AddElement(data) =>
                {
                    let fe_type = data.fe_type;
                    let nodes_numbers = data.nodes_numbers;
                    let number = data.number;
                    let properties = data.properties;
                    match self.state.fem
                        .add_element(fe_type, nodes_numbers,
                                     FEData { number, nodes: Vec::new(), properties })
                    {
                        Err(msg) => self.state.analysis_message = Some(msg),
                        _ => ()
                    }
                },
            Msg::UpdateElement(data) =>
                {
                    let nodes_numbers = data.nodes_numbers;
                    let number = data.number;
                    let properties = data.properties;
                    match self.state.fem
                        .update_element(nodes_numbers,
                            FEData { number, nodes: Vec::new(), properties })
                    {
                        Err(msg) => self.state.analysis_message = Some(msg),
                        _ => ()
                    }
                },
            Msg::DeleteElement(number) => match self.state.fem.delete_element(number)
                {
                    Err(msg) => self.state.analysis_message = Some(msg),
                    _ => (),
                },
            Msg::AddBC(data) =>
                {
                    let node_number = data.node_number;
                    let bcs = vec![
                        data.x_direction_value, data.y_direction_value,
                        data.z_direction_value, data.yz_plane_value,
                        data.zx_plane_value, data.xy_plane_value];
                    for i in 0..bcs.len()
                    {
                        if let Some(value) = bcs[i]
                        {
                            let bc_type = data.bc_type;
                            let number = data.number * GLOBAL_DOF + i as ElementsNumbers;
                            let dof_parameter =
                                GlobalDOFParameter::iterator().nth(i).unwrap();
                            match self.state.fem.add_bc(
                                bc_type, number, node_number, *dof_parameter, value)
                            {
                                Err(msg) => self.state.analysis_message = Some(msg),
                                _ => ()
                            }
                        }
                    }
                },
            Msg::UpdateBC(data) =>
                {
                    let node_number = data.node_number;
                    let bcs = vec![
                        data.x_direction_value, data.y_direction_value,
                        data.z_direction_value, data.yz_plane_value,
                        data.zx_plane_value, data.xy_plane_value];
                    for i in 0..bcs.len()
                    {
                        let bc_type = data.bc_type;
                        let number = data.number * GLOBAL_DOF + i as ElementsNumbers;
                        let dof_parameter =
                            GlobalDOFParameter::iterator().nth(i).unwrap();
                        if let Some(value) = bcs[i]
                        {
                            if self.state.fem.boundary_conditions
                                .iter()
                                .position(|model_bc|
                                    model_bc.number_same(number) && model_bc.type_same(bc_type))
                                .is_some()
                            {
                                match self.state.fem.update_bc(bc_type, number, node_number,
                                    *dof_parameter, value)
                                {
                                    Err(msg) => self.state.analysis_message = Some(msg),
                                    _ => (),
                                }
                            }
                            else
                            {
                                match self.state.fem.add_bc(
                                    bc_type, number, node_number, *dof_parameter, value)
                                {
                                    Err(msg) =>
                                        self.state.analysis_message = Some(msg),
                                    _ => (),
                                }
                            }
                        }
                        else
                        {
                            if self.state.fem.boundary_conditions
                                .iter()
                                .position(|bc|
                                    bc.number_same(number) &&
                                    bc.type_same(bc_type))
                                .is_some()
                            {
                                match self.state.fem.delete_bc(bc_type, number)
                                {
                                    Err(msg) => self.state.analysis_message = Some(msg),
                                    _ => ()
                                }
                            }
                        }
                    }
                },
            Msg::DeleteBC(data) =>
                {
                    let bcs = vec![
                        data.x_direction_value, data.y_direction_value,
                        data.z_direction_value, data.yz_plane_value,
                        data.zx_plane_value, data.xy_plane_value];
                    for i in 0..bcs.len()
                    {
                        if bcs[i].is_some()
                        {
                            let bc_type = data.bc_type;
                            let number = data.number * GLOBAL_DOF + i as ElementsNumbers;
                            match self.state.fem.delete_bc(bc_type, number)
                            {
                                Err(msg) => self.state.analysis_message = Some(msg),
                                _ => ()
                            }
                        }
                    }
                },
            Msg::AddAnalysisErrorMessage(msg) => self.state.analysis_message = Some(msg),
            Msg::ResetAnalysisMessage => self.state.analysis_message = None,
            Msg::Submit =>
                {
                    match self.submit()
                    {
                        Ok(_) => (),
                        Err(msg) => self.state.analysis_message = Some(msg),
                    }
                },
            Msg::EditFEM =>
                {
                    self.state.is_preprocessor_active = true;
                    self.state.global_displacements = Rc::new(None);
                    self.state.reactions = Rc::new(None);
                    self.state.elements_analysis_result = Rc::new(None);
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
        let handle_add_analysis_message =
            self.link.callback(|msg: String| Msg::AddAnalysisErrorMessage(msg));

        let handle_change_view = self.link.callback(|view: View| Msg::ChangeView(view));
        let handle_discard_view = self.link.callback(|_| Msg::DiscardView);

        let mut postproc_init_uid_number = 0 as UIDNumbers;

        let drawn_nodes =
            self.state.fem.drawn_nodes_rc(&mut postproc_init_uid_number);
        let handle_add_node =
            self.link.callback(|data: FEDrawnNodeData| Msg::AddNode(data));
        let handle_update_node =
            self.link.callback(|data: FEDrawnNodeData| Msg::UpdateNode(data));
        let handle_delete_node =
            self.link.callback(|number: ElementsNumbers| Msg::DeleteNode(number));

        let drawn_elements =
            self.state.fem.drawn_elements_rc(&mut postproc_init_uid_number);
        let handle_add_element =
            self.link.callback(|data: FEDrawnElementData| Msg::AddElement(data));
        let handle_update_element =
            self.link.callback(|data: FEDrawnElementData| Msg::UpdateElement(data));
        let handle_delete_element =
            self.link.callback(|number: ElementsNumbers| Msg::DeleteElement(number));

        let drawn_bcs =
            self.state.fem.drawn_bcs_rc(&mut postproc_init_uid_number);
        let handle_add_bc = self.link.callback(|data: FEDrawnBCData| Msg::AddBC(data));
        let handle_update_bc =
            self.link.callback(|data: FEDrawnBCData| Msg::UpdateBC(data));
        let handle_delete_bc =
            self.link.callback(|data: FEDrawnBCData| Msg::DeleteBC(data));

        let handle_reset_analysis_message =
            self.link.callback(|_| Msg::ResetAnalysisMessage);

        let handle_submit = self.link.callback(|_| Msg::Submit);

        let view = self.state.view.to_owned();
        let preprocessor_is_active = self.state.is_preprocessor_active.to_owned();

        let canvas_width = self.state.canvas_width.to_owned();
        let canvas_height = self.state.canvas_height.to_owned();
        let analysis_message = self.state.analysis_message.to_owned();
        let global_displacements =
            Rc::clone(&self.state.global_displacements);
        let reactions = Rc::clone(&self.state.reactions);
        let elements_analysis_result =
            Rc::clone(&self.state.elements_analysis_result);
        let handle_edit_fem = self.link.callback(|_| Msg::EditFEM);

        let render = Router::render(move |switch: AppRoute| match switch
        {
            AppRoute::Preprocessor =>
                html!
                {
                    <Preprocessor
                        view=view.to_owned(),
                        change_view=handle_change_view.to_owned(),
                        discard_view=handle_discard_view.to_owned(),
                        is_preprocessor_active=preprocessor_is_active.to_owned(),

                        drawn_nodes=Rc::clone(&drawn_nodes),
                        add_node=handle_add_node.to_owned(),
                        update_node=handle_update_node.to_owned(),
                        delete_node=handle_delete_node.to_owned(),

                        drawn_elements=Rc::clone(&drawn_elements),
                        add_element=handle_add_element.to_owned(),
                        update_element=handle_update_element.to_owned(),
                        delete_element=handle_delete_element.to_owned(),

                        drawn_bcs=Rc::clone(&drawn_bcs),
                        add_bc=handle_add_bc.to_owned(),
                        update_bc=handle_update_bc.to_owned(),
                        delete_bc=handle_delete_bc.to_owned(),
                        add_analysis_message=handle_add_analysis_message.to_owned(),

                        canvas_width=canvas_width.to_owned(),
                        canvas_height=canvas_height.to_owned(),
                        analysis_message=analysis_message.to_owned(),

                        reset_analysis_message=handle_reset_analysis_message.to_owned(),

                        submit=handle_submit.to_owned(),
                        global_displacements=Rc::clone(&global_displacements),
                        reactions=Rc::clone(&reactions),
                        elements_analysis_result=Rc::clone(&elements_analysis_result),
                        edit_fem=handle_edit_fem.to_owned(),
                    />
                },
            AppRoute::Postprocessor =>
                html!
                {
                    <Postprocessor
                        view=view.to_owned(),
                        change_view=handle_change_view.to_owned(),
                        discard_view=handle_discard_view.to_owned(),
                        canvas_width=canvas_width.to_owned(),
                        canvas_height=canvas_height.to_owned(),
                        drawn_nodes=Rc::clone(&drawn_nodes),
                        drawn_elements=Rc::clone(&drawn_elements),
                        postproc_init_uid_number=postproc_init_uid_number.to_owned(),
                        global_displacements=Rc::clone(&global_displacements),
                        reactions=Rc::clone(&reactions),
                        elements_analysis_result=Rc::clone(&elements_analysis_result),
                    />
                },
            AppRoute::HomePage =>
                html!
                {
                    <Preprocessor
                        view=view.to_owned(),
                        change_view=handle_change_view.to_owned(),
                        discard_view=handle_discard_view.to_owned(),
                        is_preprocessor_active=preprocessor_is_active.to_owned(),

                        drawn_nodes=Rc::clone(&drawn_nodes),
                        add_node=handle_add_node.to_owned(),
                        update_node=handle_update_node.to_owned(),
                        delete_node=handle_delete_node.to_owned(),

                        drawn_elements=Rc::clone(&drawn_elements),
                        add_element=handle_add_element.to_owned(),
                        update_element=handle_update_element.to_owned(),
                        delete_element=handle_delete_element.to_owned(),

                        drawn_bcs=Rc::clone(&drawn_bcs),
                        add_bc=handle_add_bc.to_owned(),
                        update_bc=handle_update_bc.to_owned(),
                        delete_bc=handle_delete_bc.to_owned(),
                        add_analysis_message=handle_add_analysis_message.to_owned(),

                        canvas_width=canvas_width.to_owned(),
                        canvas_height=canvas_height.to_owned(),
                        analysis_message=analysis_message.to_owned(),

                        reset_analysis_message=handle_reset_analysis_message.to_owned(),

                        submit=handle_submit.to_owned(),
                        global_displacements=Rc::clone(&global_displacements),
                        reactions=Rc::clone(&reactions),
                        elements_analysis_result=Rc::clone(&elements_analysis_result),
                        edit_fem=handle_edit_fem.to_owned(),
                    />
                },
        });

        html!
        {
            <main class={ MAIN_CLASS }>
                <div class={ MAIN_CONTAINER_CLASS }>
                    <Router<AppRoute, ()> render=render />
                </div>
            </main>
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
