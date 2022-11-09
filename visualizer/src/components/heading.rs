use super::heading_css::*;
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
            <header class={heading()}>
                <h1 class={page_title()}>
                    {"Wave Collapse Function Visliazer"}
                </h1>
                <span class={page_description()}>
                    {"Trying to implement WCF in Rust 🦀"}
                </span>
            </header>
        }
    }
}
