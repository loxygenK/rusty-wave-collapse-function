use std::{marker::PhantomData, rc::Rc};

use f4n_wcf_core::field::Field;
use yew::prelude::*;

use crate::{
    components::{Heading, FieldGrid},
    js::console, renderable_tiles::RenderableTileId
};

pub struct IndexPage<Id> {
    _phantom: PhantomData<Id>
}

#[derive(PartialEq, Eq, Properties)]
pub struct IndexPageProps<Id: RenderableTileId> {
    pub field: Rc<Field<'static, Id>>
}

impl<Id: RenderableTileId> Component for IndexPage<Id> {
    type Message = ();
    type Properties = IndexPageProps<Id>;

    fn create(_ctx: &Context<Self>) -> Self {
        IndexPage { _phantom: PhantomData }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        console::log("Hello, world!");

        let field = ctx.props().field.clone();

        html! {
            <>
                <Heading />
                <FieldGrid<Id> field={field} />
            </>
        }
    }
}
