use std::rc::Rc;

use f4n_wcf_core::field::Field;
use pages::index::IndexPage;
use renderable_tiles::RenderableTileId;
use yew::props;

mod components;
mod js;
mod macros;
mod pages;
pub mod renderable_tiles;

pub fn start<Id: RenderableTileId>(field: Field<'static, Id>) {
    yew::start_app_with_props::<IndexPage<Id>>(props! {
        pages::index::IndexPageProps<Id> {
            field: Rc::new(field)
        }
    });
}
