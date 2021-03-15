use yew::prelude::*;
use yew_router::prelude::RouterAnchor;

use crate::route::AppRoute;


pub struct Postprocessor;


impl Component for Postprocessor
{
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self
    {
        Self {  }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender
    {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender
    {
        false
    }

    fn view(&self) -> Html
    {
        type Anchor = RouterAnchor<AppRoute>;

        html!
        {
            <>
                <h2>{ "Postprocessor" }</h2>
                <Anchor route=AppRoute::Preprocessor>
                    <button class="button">{ "Preprocessor" }</button>
                </Anchor>
            </>
        }
    }
}