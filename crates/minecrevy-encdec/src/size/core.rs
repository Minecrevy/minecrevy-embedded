use crate::{WireSize, options::IntOptions, var_i32_size};

macro_rules! impl_primitive {
    ($($ty:ty),*) => {
        $(
            impl WireSize for $ty {
                type Options = ();

                #[inline]
                fn wire_size(&self, (): Self::Options) -> usize {
                    core::mem::size_of::<Self>()
                }
            }
        )*
    };
}

impl_primitive!(u8, i8, u16, i16, u32, u64, i64, f32, f64);

impl WireSize for i32 {
    type Options = IntOptions;

    #[inline]
    fn wire_size(&self, IntOptions { varint }: Self::Options) -> usize {
        if varint {
            var_i32_size(*self)
        } else {
            core::mem::size_of::<Self>()
        }
    }
}
