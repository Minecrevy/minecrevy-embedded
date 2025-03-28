mod core;
mod heapless;

pub use minecrevy_encdec_macros::WireSize;

pub trait WireSize {
    type Options: Clone + Default;

    fn wire_size(&self, options: Self::Options) -> usize;
}
