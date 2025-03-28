use heapless::String;

use crate::{WireSize, var_i32_size};

impl<const N: usize> WireSize for String<N> {
    type Options = ();

    fn wire_size(&self, (): Self::Options) -> usize {
        var_i32_size(i32::try_from(self.len()).unwrap()) + self.len()
    }
}
