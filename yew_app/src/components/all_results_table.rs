use yew::prelude::*;

use crate::
    {
        AuxDisplacement, AnalysisResult, AxisComponent, Force, FeNode,
        Displacement, AuxElement, StrainStressComponent
    };


#[derive(Properties, PartialEq, Clone)]
pub struct Props
{
    pub nodes: Vec<FeNode<u16, f64>>,
    pub aux_elements: Vec<AuxElement>,
    pub aux_displacements: Vec<AuxDisplacement>,
    pub analysis_result: AnalysisResult,
}




pub struct AllResultsTable
{
    props: Props,
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
            <>
                <table>
                    <thead>
                        <tr>
                            <td rowspan=2></td>
                            <th colspan=6>{ "Reactions" }</th>
                        </tr>
                        <tr>
                            <td>{ "Rx" }</td>
                            <td>{ "Ry" }</td>
                            <td>{ "Rz" }</td>
                            <td>{ "Mxy" }</td>
                            <td>{ "Myz" }</td>
                            <td>{ "Mzx" }</td>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            for self.props.aux_displacements.iter().map(|aux_displacement|
                            html!
                            {
                                <tr>
                                    <td>{ format!("node {}", aux_displacement.node_number) }</td>
                                    <td>
                                        {
                                            if let Some(r_x_value) =
                                                self.props.analysis_result.reactions
                                                    .get(&Force
                                                        {
                                                            node_number: aux_displacement.node_number,
                                                            component: AxisComponent::U
                                                        })
                                            {
                                                r_x_value.to_string()
                                            }
                                            else
                                            {
                                                "N/A".to_string()
                                            }
                                        }
                                    </td>
                                    <td>
                                        {
                                            if let Some(r_y_value) =
                                                self.props.analysis_result.reactions
                                                    .get(&Force
                                                        {
                                                            node_number: aux_displacement.node_number,
                                                            component: AxisComponent::V
                                                        })
                                            {
                                                r_y_value.to_string()
                                            }
                                            else
                                            {
                                                "N/A".to_string()
                                            }
                                        }
                                    </td>
                                    <td>
                                        {
                                            if let Some(r_z_value) =
                                                self.props.analysis_result.reactions
                                                    .get(&Force
                                                        {
                                                            node_number: aux_displacement.node_number,
                                                            component: AxisComponent::W
                                                        })
                                            {
                                                r_z_value.to_string()
                                            }
                                            else
                                            {
                                                "N/A".to_string()
                                            }
                                        }
                                    </td>
                                    <td>
                                        {
                                            if let Some(m_xy_value) =
                                                self.props.analysis_result.reactions
                                                    .get(&Force
                                                        {
                                                            node_number: aux_displacement.node_number,
                                                            component: AxisComponent::ThetaW
                                                        })
                                            {
                                                m_xy_value.to_string()
                                            }
                                            else
                                            {
                                                "N/A".to_string()
                                            }
                                        }
                                    </td>
                                    <td>
                                        {
                                            if let Some(m_yz_value) =
                                                self.props.analysis_result.reactions
                                                    .get(&Force
                                                        {
                                                            node_number: aux_displacement.node_number,
                                                            component: AxisComponent::ThetaU
                                                        })
                                            {
                                                m_yz_value.to_string()
                                            }
                                            else
                                            {
                                                "N/A".to_string()
                                            }
                                        }
                                    </td>
                                    <td>
                                        {
                                            if let Some(m_zx_value) =
                                                self.props.analysis_result.reactions
                                                    .get(&Force
                                                        {
                                                            node_number: aux_displacement.node_number,
                                                            component: AxisComponent::ThetaV
                                                        })
                                            {
                                                m_zx_value.to_string()
                                            }
                                            else
                                            {
                                                "N/A".to_string()
                                            }
                                        }
                                    </td>
                                </tr>
                            })
                        }
                    </tbody>
                </table>
                <table>
                    <thead>
                        <tr>
                            <td rowspan=2></td>
                            <th colspan=6>{ "Displacements" }</th>
                        </tr>
                        <tr>
                            <td>{ "Ux" }</td>
                            <td>{ "Uy" }</td>
                            <td>{ "Uz" }</td>
                            <td>{ "Theta_xy" }</td>
                            <td>{ "Theta_yz" }</td>
                            <td>{ "Theta_zx" }</td>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            for self.props.nodes.iter().map(|node|
                            html!
                            {
                                <tr>
                                    <td>{ format!("node {}", node.number) }</td>
                                    <td>
                                        {
                                            if let Some(u_x_value) =
                                                self.props.analysis_result.displacements
                                                    .get(&Displacement
                                                        {
                                                            node_number: node.number,
                                                            component: AxisComponent::U
                                                        })
                                            {
                                                u_x_value.to_string()
                                            }
                                            else
                                            {
                                                "N/A".to_string()
                                            }
                                        }
                                    </td>
                                    <td>
                                        {
                                            if let Some(u_y_value) =
                                                self.props.analysis_result.displacements
                                                    .get(&Displacement
                                                        {
                                                            node_number: node.number,
                                                            component: AxisComponent::V
                                                        })
                                            {
                                                u_y_value.to_string()
                                            }
                                            else
                                            {
                                                "N/A".to_string()
                                            }
                                        }
                                    </td>
                                    <td>
                                        {
                                            if let Some(u_z_value) =
                                                self.props.analysis_result.displacements
                                                    .get(&Displacement
                                                        {
                                                            node_number: node.number,
                                                            component: AxisComponent::W
                                                        })
                                            {
                                                u_z_value.to_string()
                                            }
                                            else
                                            {
                                                "N/A".to_string()
                                            }
                                        }
                                    </td>
                                    <td>
                                        {
                                            if let Some(theta_xy_value) =
                                                self.props.analysis_result.displacements
                                                    .get(&Displacement
                                                        {
                                                            node_number: node.number,
                                                            component: AxisComponent::ThetaW
                                                        })
                                            {
                                                theta_xy_value.to_string()
                                            }
                                            else
                                            {
                                                "N/A".to_string()
                                            }
                                        }
                                    </td>
                                    <td>
                                        {
                                            if let Some(theta_yz_value) =
                                                self.props.analysis_result.displacements
                                                    .get(&Displacement
                                                        {
                                                            node_number: node.number,
                                                            component: AxisComponent::ThetaU
                                                        })
                                            {
                                                theta_yz_value.to_string()
                                            }
                                            else
                                            {
                                                "N/A".to_string()
                                            }
                                        }
                                    </td>
                                    <td>
                                        {
                                            if let Some(theta_zx_value) =
                                                self.props.analysis_result.displacements
                                                    .get(&Displacement
                                                        {
                                                            node_number: node.number,
                                                            component: AxisComponent::ThetaV
                                                        })
                                            {
                                                theta_zx_value.to_string()
                                            }
                                            else
                                            {
                                                "N/A".to_string()
                                            }
                                        }
                                    </td>
                                </tr>
                            })
                        }
                    </tbody>
                </table>
                <table>
                    <thead>
                        <tr>
                            <td rowspan=2></td>
                            <th colspan=6>{ "Strains" }</th>
                        </tr>
                        <tr>
                            <td>{ "XX" }</td>
                            <td>{ "YY" }</td>
                            <td>{ "ZZ" }</td>
                            <td>{ "XY" }</td>
                            <td>{ "YZ" }</td>
                            <td>{ "ZX" }</td>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            for self.props.aux_elements.iter().map(|aux_element|
                            html!
                            {
                                <tr>
                                    <td>{ format!("element {}", aux_element.number) }</td>
                                    <td>
                                        {
                                            if let Some(strains_and_stresses) =
                                                self.props.analysis_result.strains_and_stresses
                                                    .get(&aux_element.number)
                                            {
                                                if let Some(position) = strains_and_stresses
                                                    .iter()
                                                    .position(|strain_stress| strain_stress.strain.component == StrainStressComponent::XX)
                                                {
                                                    strains_and_stresses[position].strain.value.to_string()
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
                                    </td>
                                    <td>
                                        {
                                            if let Some(strains_and_stresses) =
                                                self.props.analysis_result.strains_and_stresses
                                                    .get(&aux_element.number)
                                            {
                                                if let Some(position) = strains_and_stresses
                                                    .iter()
                                                    .position(|strain_stress| strain_stress.strain.component == StrainStressComponent::YY)
                                                {
                                                    strains_and_stresses[position].strain.value.to_string()
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
                                    </td>
                                    <td>
                                        {
                                            if let Some(strains_and_stresses) =
                                                self.props.analysis_result.strains_and_stresses
                                                    .get(&aux_element.number)
                                            {
                                                if let Some(position) = strains_and_stresses
                                                    .iter()
                                                    .position(|strain_stress| strain_stress.strain.component == StrainStressComponent::ZZ)
                                                {
                                                    strains_and_stresses[position].strain.value.to_string()
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
                                    </td>
                                    <td>
                                        {
                                            if let Some(strains_and_stresses) =
                                                self.props.analysis_result.strains_and_stresses
                                                    .get(&aux_element.number)
                                            {
                                                if let Some(position) = strains_and_stresses
                                                    .iter()
                                                    .position(|strain_stress| strain_stress.strain.component == StrainStressComponent::XY)
                                                {
                                                    strains_and_stresses[position].strain.value.to_string()
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
                                    </td>
                                    <td>
                                        {
                                            if let Some(strains_and_stresses) =
                                                self.props.analysis_result.strains_and_stresses
                                                    .get(&aux_element.number)
                                            {
                                                if let Some(position) = strains_and_stresses
                                                    .iter()
                                                    .position(|strain_stress| strain_stress.strain.component == StrainStressComponent::YZ)
                                                {
                                                    strains_and_stresses[position].strain.value.to_string()
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
                                    </td>
                                    <td>
                                        {
                                            if let Some(strains_and_stresses) =
                                                self.props.analysis_result.strains_and_stresses
                                                    .get(&aux_element.number)
                                            {
                                                if let Some(position) = strains_and_stresses
                                                    .iter()
                                                    .position(|strain_stress| strain_stress.strain.component == StrainStressComponent::ZX)
                                                {
                                                    strains_and_stresses[position].strain.value.to_string()
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
                                    </td>
                                </tr>
                            })
                        }
                    </tbody>
                </table>
                <table>
                    <thead>
                        <tr>
                            <td rowspan=2></td>
                            <th colspan=6>{ "Stresses" }</th>
                        </tr>
                        <tr>
                            <td>{ "XX" }</td>
                            <td>{ "YY" }</td>
                            <td>{ "ZZ" }</td>
                            <td>{ "XY" }</td>
                            <td>{ "YZ" }</td>
                            <td>{ "ZX" }</td>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            for self.props.aux_elements.iter().map(|aux_element|
                            html!
                            {
                                <tr>
                                    <td>{ format!("element {}", aux_element.number) }</td>
                                    <td>
                                        {
                                            if let Some(strains_and_stresses) =
                                                self.props.analysis_result.strains_and_stresses
                                                    .get(&aux_element.number)
                                            {
                                                if let Some(position) = strains_and_stresses
                                                    .iter()
                                                    .position(|strain_stress| strain_stress.stress.component == StrainStressComponent::XX)
                                                {
                                                    strains_and_stresses[position].stress.value.to_string()
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
                                    </td>
                                    <td>
                                        {
                                            if let Some(strains_and_stresses) =
                                                self.props.analysis_result.strains_and_stresses
                                                    .get(&aux_element.number)
                                            {
                                                if let Some(position) = strains_and_stresses
                                                    .iter()
                                                    .position(|strain_stress| strain_stress.stress.component == StrainStressComponent::YY)
                                                {
                                                    strains_and_stresses[position].stress.value.to_string()
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
                                    </td>
                                    <td>
                                        {
                                            if let Some(strains_and_stresses) =
                                                self.props.analysis_result.strains_and_stresses
                                                    .get(&aux_element.number)
                                            {
                                                if let Some(position) = strains_and_stresses
                                                    .iter()
                                                    .position(|strain_stress| strain_stress.stress.component == StrainStressComponent::ZZ)
                                                {
                                                    strains_and_stresses[position].stress.value.to_string()
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
                                    </td>
                                    <td>
                                        {
                                            if let Some(strains_and_stresses) =
                                                self.props.analysis_result.strains_and_stresses
                                                    .get(&aux_element.number)
                                            {
                                                if let Some(position) = strains_and_stresses
                                                    .iter()
                                                    .position(|strain_stress| strain_stress.stress.component == StrainStressComponent::XY)
                                                {
                                                    strains_and_stresses[position].stress.value.to_string()
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
                                    </td>
                                    <td>
                                        {
                                            if let Some(strains_and_stresses) =
                                                self.props.analysis_result.strains_and_stresses
                                                    .get(&aux_element.number)
                                            {
                                                if let Some(position) = strains_and_stresses
                                                    .iter()
                                                    .position(|strain_stress| strain_stress.stress.component == StrainStressComponent::YZ)
                                                {
                                                    strains_and_stresses[position].stress.value.to_string()
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
                                    </td>
                                    <td>
                                        {
                                            if let Some(strains_and_stresses) =
                                                self.props.analysis_result.strains_and_stresses
                                                    .get(&aux_element.number)
                                            {
                                                if let Some(position) = strains_and_stresses
                                                    .iter()
                                                    .position(|strain_stress| strain_stress.stress.component == StrainStressComponent::ZX)
                                                {
                                                    strains_and_stresses[position].stress.value.to_string()
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
                                    </td>
                                </tr>
                            })
                        }
                    </tbody>
                </table>
            </>
        }
    }
}
