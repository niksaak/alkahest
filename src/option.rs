use crate::schema::{Pack, Packed, Schema, SchemaUnpack, Unpacked};

impl<'a, T> SchemaUnpack<'a> for Option<T>
where
    T: Schema,
{
    type Unpacked = Option<Unpacked<'a, T>>;
}

#[derive(Copy)]
#[repr(C, packed)]
pub struct PackedOption<T: bytemuck::Pod> {
    some: u8,
    value: T,
}

impl<T: bytemuck::Pod> Clone for PackedOption<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

// `bytemuck` must be able to derive those safely. See https://github.com/Lokathor/bytemuck/issues/70
#[allow(unsafe_code)]
unsafe impl<T: bytemuck::Pod> bytemuck::Zeroable for PackedOption<T> {}
#[allow(unsafe_code)]
unsafe impl<T: bytemuck::Pod> bytemuck::Pod for PackedOption<T> {}

impl<T> Schema for Option<T>
where
    T: Schema,
{
    type Packed = PackedOption<T::Packed>;

    #[inline]
    fn align() -> usize {
        T::align()
    }

    #[inline]
    fn unpack<'a>(packed: PackedOption<T::Packed>, input: &'a [u8]) -> Unpacked<'a, Self> {
        if packed.some != 0 {
            Some(T::unpack(packed.value, input))
        } else {
            None
        }
    }
}

impl<T, U> Pack<Option<T>> for Option<U>
where
    T: Schema,
    U: Pack<T>,
{
    #[inline]
    fn pack(self, offset: usize, output: &mut [u8]) -> Result<(Packed<Option<T>>, usize), usize> {
        match self {
            None => Ok(
                (
                    PackedOption {
                        some: 0,
                        value: bytemuck::Zeroable::zeroed(),
                    },
                    0,
                )
            ),
            Some(value) => match value.pack(offset, output) {
                Ok((packed, used)) => Ok(
                    (
                        PackedOption {
                            some: 1,
                            value: packed,
                        },
                        used,
                    )
                ),
                Err(need) => Err(need),
            }
        }
    }
}
