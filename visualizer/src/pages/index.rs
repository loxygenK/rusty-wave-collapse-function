use yew::prelude::*;
use crate::components::heading::Heading;

pub struct IndexPage;

impl Component for IndexPage {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        IndexPage
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <Heading />
        }
    }
}
