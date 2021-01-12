use yew::prelude::*;

use crate::
    {
        AuxDisplacement, AnalysisResult, AxisComponent, Force, FeNode,
        Displacement, AuxElement, StrainStressComponent
    };


const ALL_RESULTS_TABLE_CONTAINER_CLASS: &str = "all_results_table";
const COMPONENT_NAME_CLASS: &str = "component_name";


#[derive(Properties, PartialEq, Clone)]
pub struct Props
{
    pub nodes: Vec<FeNode<u16, f64>>,
    pub aux_elements: Vec<AuxElement>,
    pub aux_displacements: Vec<AuxDisplacement>,
    pub analysis_result: AnalysisResult,
    pub canvas_width: u32,
}


pub struct AllResultsTable
{
    props: Props,
}


impl AllResultsTable
{
    fn get_reaction_value(&self, node_number: u16, component: AxisComponent) -> String
    {
        if let Some(reaction_value) =
            self.props.analysis_result.reactions
                .get(&Force { node_number, component })
        {
            format!("{:+.3e}", reaction_value)
        }
        else
        {
            "N/A".to_string()
        }
    }


    fn get_displacement_value(&self, node_number: u16, component: AxisComponent) -> String
    {
        if let Some(displacement_value) =
            self.props.analysis_result.displacements
                .get(&Displacement { node_number, component })
        {
            format!("{:+.3e}", displacement_value)
        }
        else
        {
            "N/A".to_string()
        }
    }


    fn get_strain_value(&self, element_number: u16, component: StrainStressComponent) -> String
    {
        if let Some(strains_and_stresses) =
            self.props.analysis_result.strains_and_stresses
                .get(&element_number)
        {
            if let Some(position) = strains_and_stresses
                .iter()
                .position(|strain_stress| strain_stress.strain.component == component)
            {
                format!("{:+.3e}", strains_and_stresses[position].strain.value)
            }
            else
            {
                "N/A".to_string()
            }
        }
        else
        {
            "N/A".to_string()
        }
    }


    fn get_stress_value(&self, element_number: u16, component: StrainStressComponent) -> String
    {
        if let Some(strains_and_stresses) =
            self.props.analysis_result.strains_and_stresses
                .get(&element_number)
        {
            if let Some(position) = strains_and_stresses
                .iter()
                .position(|strain_stress| strain_stress.stress.component == component)
            {
                format!("{:+.3e}", strains_and_stresses[position].stress.value)
            }
            else
            {
                "N/A".to_string()
            }
        }
        else
        {
            "N/A".to_string()
        }
    }
}


impl Component for AllResultsTable
{
    type Message = ();
    type Properties = Props;


    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self
    {
        Self { props }
    }


    fn update(&mut self, _msg: Self::Message) -> ShouldRender
    {
        true
    }


    fn change(&mut self, props: Self::Properties) -> ShouldRender
    {
        if self.props != props
        {
            self.props = props;
            true
        }
        else
        {
            false
        }
    }

    fn view(&self) -> Html
    {
        html!
        {
            <div class={ ALL_RESULTS_TABLE_CONTAINER_CLASS }>
                <table style={ format!("width: {}px;", self.props.canvas_width) }>
                    <thead>
                        <tr>
                            <td rowspan=2></td>
                            <th colspan=6>{ "Reaction" }</th>
                        </tr>
                        <tr>
                            <td
                                class={ COMPONENT_NAME_CLASS },
                                style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }
                            >
                                { "Rx" }
                            </td>
                            <td
                                class={ COMPONENT_NAME_CLASS },
                                style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }
                            >
                                { "Ry" }
                            </td>
                            <td
                                class={ COMPONENT_NAME_CLASS },
                                style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }
                            >
                                { "Rz" }
                            </td>
                            <td
                                class={ COMPONENT_NAME_CLASS },
                                style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }
                            >
                                { "Mxy" }
                            </td>
                            <td
                                class={ COMPONENT_NAME_CLASS },
                                style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }
                            >
                                { "Myz" }
                            </td>
                            <td
                                class={ COMPONENT_NAME_CLASS },
                                style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }
                            >
                                { "Mzx" }
                            </td>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            for self.props.aux_displacements.iter().map(|aux_displacement|
                            html!
                            {
                                <tr>
                                    <td
                                        style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }
                                    >
                                        { format!("node {}", aux_displacement.node_number) }
                                    </td>
                                    {
                                        for AxisComponent::iterator().map(|component|
                                        html!
                                        {
                                            <td style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }>
                                                { self.get_reaction_value(aux_displacement.node_number, *component) }
                                            </td>
                                        })
                                    }
                                </tr>
                            })
                        }
                    </tbody>
                </table>
                <table style={ format!("width: {}px;", self.props.canvas_width) }>
                    <thead>
                        <tr>
                            <td rowspan=2></td>
                            <th colspan=6>{ "Displacement" }</th>
                        </tr>
                        <tr>
                            <td
                                class={ COMPONENT_NAME_CLASS },
                                style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }
                            >
                                { "Ux" }
                            </td>
                            <td
                                class={ COMPONENT_NAME_CLASS },
                                style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }
                            >
                                { "Uy" }
                            </td>
                            <td
                                class={ COMPONENT_NAME_CLASS },
                                style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }
                            >
                                { "Uz" }
                            </td>
                            <td
                                class={ COMPONENT_NAME_CLASS },
                                style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }
                            >
                                { "Theta_xy" }
                            </td>
                            <td
                                class={ COMPONENT_NAME_CLASS },
                                style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }
                            >
                                { "Theta_yz" }
                            </td>
                            <td
                                class={ COMPONENT_NAME_CLASS },
                                style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }
                            >
                                { "Theta_zx" }
                            </td>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            for self.props.nodes.iter().map(|node|
                            html!
                            {
                                <tr>
                                    <td
                                        style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }
                                    >
                                        { format!("node {}", node.number) }
                                    </td>
                                    {
                                        for AxisComponent::iterator().map(|component|
                                        html!
                                        {
                                            <td style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }>
                                                { self.get_displacement_value(node.number, *component) }
                                            </td>
                                        })
                                    }
                                </tr>
                            })
                        }
                    </tbody>
                </table>
                <table style={ format!("width: {}px;", self.props.canvas_width) }>
                    <thead>
                        <tr>
                            <td rowspan=2></td>
                            <th colspan=6>{ "Strain" }</th>
                        </tr>
                        <tr>
                            <td
                                class={ COMPONENT_NAME_CLASS },
                                style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }
                            >
                                { "XX" }
                            </td>
                            <td
                                class={ COMPONENT_NAME_CLASS },
                                style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }
                            >
                                { "YY" }
                            </td>
                            <td
                                class={ COMPONENT_NAME_CLASS },
                                style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }
                            >
                                { "ZZ" }
                            </td>
                            <td
                                class={ COMPONENT_NAME_CLASS },
                                style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }
                            >
                                { "XY" }
                            </td>
                            <td
                                class={ COMPONENT_NAME_CLASS },
                                style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }
                            >
                                { "YZ" }
                            </td>
                            <td
                                class={ COMPONENT_NAME_CLASS },
                                style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }
                            >
                                { "ZX" }
                            </td>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            for self.props.aux_elements.iter().map(|aux_element|
                            html!
                            {
                                <tr>
                                    <td
                                        style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }
                                    >
                                        { format!("element {}", aux_element.number) }
                                    </td>
                                    {
                                        for StrainStressComponent::iterator().map(|component|
                                        html!
                                        {
                                            <td style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }>
                                                { self.get_strain_value(aux_element.number, *component) }
                                            </td>
                                        })
                                    }
                                </tr>
                            })
                        }
                    </tbody>
                </table>
                <table style={ format!("width: {}px;", self.props.canvas_width) }>
                    <thead>
                        <tr>
                            <td rowspan=2></td>
                            <th colspan=6>{ "Stress" }</th>
                        </tr>
                        <tr>
                            <td
                                class={ COMPONENT_NAME_CLASS },
                                style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }
                            >
                                { "XX" }
                            </td>
                            <td
                                class={ COMPONENT_NAME_CLASS },
                                style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }
                            >
                                { "YY" }
                            </td>
                            <td
                                class={ COMPONENT_NAME_CLASS },
                                style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }
                            >
                                { "ZZ" }
                            </td>
                            <td
                                class={ COMPONENT_NAME_CLASS },
                                style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }
                            >
                                { "XY" }
                            </td>
                            <td
                                class={ COMPONENT_NAME_CLASS },
                                style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }
                            >
                                { "YZ" }
                            </td>
                            <td
                                class={ COMPONENT_NAME_CLASS },
                                style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }
                            >
                                { "ZX" }
                            </td>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            for self.props.aux_elements.iter().map(|aux_element|
                            html!
                            {
                                <tr>
                                    <td
                                        style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }
                                    >
                                        { format!("element {}", aux_element.number) }
                                    </td>
                                    {
                                        for StrainStressComponent::iterator().map(|component|
                                        html!
                                        {
                                            <td style={ format!("width: {}px;", self.props.canvas_width / 7.0 as u32) }>
                                                { self.get_stress_value(aux_element.number, *component) }
                                            </td>
                                        })
                                    }
                                </tr>
                            })
                        }
                    </tbody>
                </table>
            </div>
        }
    }
}
