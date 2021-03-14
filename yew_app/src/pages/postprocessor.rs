use yew::prelude::*;
use yew_router::prelude::RouterAnchor;

use crate::route::AppRoute;


#[derive(Properties, PartialEq, Clone)]
pub struct Props
{
    pub reset_model: Callback<()>,
}

pub struct Postprocessor
{
    link: ComponentLink<Self>,
    props: Props,
}


pub enum Msg
{
    ResetModel
}

impl Component for Postprocessor
{
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self
    {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender
    {
        match msg
        {
            Msg::ResetModel => self.props.reset_model.emit(()),
        }
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
        type Anchor = RouterAnchor<AppRoute>;

        html!
        {
            <>
                <h2>{ "Postprocessor" }</h2>
                <button onclick=self.link.callback(|_| Msg::ResetModel)>{ "Reset Model" }</button>

                <Anchor route=AppRoute::HomePage>
                    <button class="button">{ "Preprocessor" }</button>
                </Anchor>
            </>
        }
    }
}