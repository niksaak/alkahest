/// The purpose of this trait is to define associated type for each lifetime for [`Schema`] trait implementations.
/// This is a workaround for lack of HRTB support for associated types.
pub trait SchemaUnpack<'a> {
    /// Unpacked value type.
    type Unpacked;
}

/// Trait for data schemas.
///
/// This trait requires implementation of [`SchemaUnpack`] trait for all lifetimes.
pub trait Schema: for<'a> SchemaUnpack<'a> + 'static {
    /// Packed value with this schema.
    /// Trivially readable from and writable to bytes.
    type Packed: bytemuck::Pod;

    /// Alignment required for successful unpacking.
    /// See [`Self::unpack`] method.
    fn align() -> usize;

    /// Unpack the value from packed value and bytes.
    /// `input` must be aligned according to [`Self::align`].
    fn unpack<'a>(packed: Self::Packed, input: &'a [u8]) -> Unpacked<'a, Self>;
}

/// Trait for packable types that match specified [`Schema`].
pub trait Pack<T: Schema> {
    /// Packs into trivially serializable value.
    ///
    /// Returns packed data and number of bytes used from `output` storage,
    /// or the required size of the output buffer in case it is not big enough.
    fn pack(self, offset: usize, output: &mut [u8]) -> Result<(Packed<T>, usize), usize>;
}

/// Type alias for packed value with [`Schema`] of type `T`.
pub type Packed<T> = <T as Schema>::Packed;

/// Type alias for unpacked value with [`Schema`] of type `T`.
pub type Unpacked<'a, T> = <T as SchemaUnpack<'a>>::Unpacked;
