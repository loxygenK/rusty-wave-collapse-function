use std::{marker::PhantomData, rc::Rc};

use f4n_wcf_core::field::Field;
use itertools::Itertools;
use yew::prelude::*;

use crate::{components::GridTile, renderable_tiles::RenderableTileId};

use super::field_grid_css::*;

pub struct FieldGrid<Id: RenderableTileId> {
    _phantom: PhantomData<Id>,
}

#[derive(Properties, PartialEq, Eq)]
pub struct FieldGridProps<Id: RenderableTileId> {
    pub field: Rc<Field<'static, Id>>,
}

impl<Id: RenderableTileId> Component for FieldGrid<Id> {
    type Message = ();
    type Properties = FieldGridProps<Id>;

    fn create(_ctx: &Context<Self>) -> Self {
        FieldGrid {
            _phantom: PhantomData,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let field = &ctx.props().field;

        let tile = (0..(field.width))
            .cartesian_product(0..(field.height))
            .into_iter()
            .map(|coord| {
                ctx.props().field
                    .at(coord)
                    .unwrap()
                    .get_collapsed()
                    .ok()
                    .flatten()
                    .and_then(|id| field.tiles.get(id))
                    .map(|tile| html!(
                        <ul>
                            <GridTile
                                image={format!("/public/{}", tile.identifier().get_image_path())}
                            />
                        </ul>
                    ))
                    .unwrap_or_else(|| html!(<ul></ul>))
            })
            .collect::<Vec<_>>();

        html! {
            <ul class={field_grid(field.width, field.height)}>
                {tile}
            </ul>
        }
    }
}
