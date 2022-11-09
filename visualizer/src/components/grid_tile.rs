use yew::prelude::*;

use super::grid_tile_css::tile_image;

pub struct GridTile;

#[derive(PartialEq, Eq, Properties)]
pub struct GridTileProps {
    pub image: String,
}

impl Component for GridTile {
    type Message = ();
    type Properties = GridTileProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        GridTile
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <img
                src={ctx.props().image.to_string()}
                class={tile_image()}
            />
        }
    }
}
