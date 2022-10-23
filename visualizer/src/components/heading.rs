use yew::prelude::*;

pub struct Heading;

impl Component for Heading {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Heading
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <h1>
                {"Wave Collapse Function Visliazer"}
            </h1>
        }
    }
}
