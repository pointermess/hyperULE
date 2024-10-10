/// hyperULE Generator / Backend
///
///

pub trait HyperBackend<I, O> {
    fn generate(source : I) -> O;
}

pub trait HyperNode<O> {
    fn get_buffer() -> O;
    fn set_buffer(buffer: O);
    fn generate();
    fn optimize();
}
