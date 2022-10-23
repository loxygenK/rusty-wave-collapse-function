use pages::index::IndexPage;

mod components;
mod pages;

pub fn start() {
    yew::start_app::<IndexPage>();
}
