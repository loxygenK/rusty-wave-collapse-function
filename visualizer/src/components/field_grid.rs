use yew::prelude::*;
use itertools::Itertools;

use super::field_grid_css::*;

pub struct FieldGrid;

#[derive(Properties, PartialEq, Eq)]
pub struct FieldGridProps {
    pub width: usize,
    pub height: usize
}

impl Component for FieldGrid {
    type Message = ();
    type Properties = FieldGridProps;

    fn create(_ctx: &Context<Self>) -> Self {
        FieldGrid
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let tile = (0..(ctx.props().width)).cartesian_product(0..(ctx.props().height))
            .into_iter()
            .map(|(x, y)| html!(<li>{x + 1}{","}{y + 1}</li>))
            .collect::<Vec<_>>();

        html! {
            <ul class={field(ctx.props().width, ctx.props().height)}>
                {tile}
            </ul>
        }
    }
}
