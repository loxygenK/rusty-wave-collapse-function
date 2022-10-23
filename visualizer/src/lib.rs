use pages::index::IndexPage;

mod components;
mod pages;
pub mod renderable_tiles;

pub fn start() {
    yew::start_app::<IndexPage>();
}
