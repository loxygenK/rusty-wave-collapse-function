use std::marker::PhantomData;

use yew::prelude::*;
use crate::components::heading::Heading;

pub struct IndexPage<Id> {
    _phantom: PhantomData<Id>,
}

#[derive(PartialEq, Eq, Properties)]
pub struct IndexPageProps<Id: Eq + Clone> {
    pub field: Vec<Vec<Option<Id>>>
}

impl<Id: Eq + Clone + 'static> Component for IndexPage<Id> {
    type Message = ();
    type Properties = IndexPageProps<Id>;

    fn create(_ctx: &Context<Self>) -> Self {
        IndexPage { _phantom: PhantomData }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <Heading />
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn destroy(&mut self, ctx: &Context<Self>) {}
}
