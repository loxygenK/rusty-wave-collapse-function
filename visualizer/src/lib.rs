use f4n_wcf_core::field::Field;
use pages::index::IndexPage;
use yew::props;

mod components;
mod js;
mod macros;
pub mod renderable_tiles;
mod pages;

pub fn start<Id: Eq + Clone>(field: Field<'static, Id>) {
    yew::start_app_with_props::<IndexPage<Id>>(props! {
        pages::index::IndexPageProps<Id> { field }
    });
}
