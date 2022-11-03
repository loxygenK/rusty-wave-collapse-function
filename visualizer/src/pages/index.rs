use std::marker::PhantomData;

use f4n_wcf_core::field::Field;
use yew::prelude::*;

use crate::{
    components::{Heading, FieldGrid},
    js::console
};

pub struct IndexPage<Id> {
    _phantom: PhantomData<Id>
}

#[derive(PartialEq, Eq, Properties)]
pub struct IndexPageProps<Id: Eq + Clone + 'static> {
    pub field: Field<'static, Id>
}

impl<Id: Eq + Clone + 'static> Component for IndexPage<Id> {
    type Message = ();
    type Properties = IndexPageProps<Id>;

    fn create(_ctx: &Context<Self>) -> Self {
        IndexPage { _phantom: PhantomData }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        console::log("Hello, world!");

        let field = &ctx.props().field;

        html! {
            <>
                <Heading />
                <FieldGrid
                    width={field.width()}
                    height={field.height()}
                />
            </>
        }
    }
}
