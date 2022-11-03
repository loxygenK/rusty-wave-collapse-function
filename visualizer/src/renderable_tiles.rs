pub trait RenderableTileId: Eq + Clone + 'static {
    fn get_image_path(&self) -> String;
}
