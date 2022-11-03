use f4n_wcf_core::field::Field;
use pages::index::IndexPage;

mod components;
mod pages;
pub mod renderable_tiles;

pub fn start<Id: Eq + Clone>(field: Field<'static, Id>) {
    yew::start_app_with_props::<IndexPage<Id>>(pages::index::IndexPageProps { field: field.to_id_vec() });
}
