use std::marker::PhantomData;

use bevy::prelude::*;
use ndshape::{ConstPow2Shape2usize, ConstPow2Shape3usize, ConstPow2Shape4usize, Shape};

/// Implementing the `Sampleable` trait means that some `N`-dimensional data
/// structure can be accessed at a coordinate value and return data. This trait
/// is a subset of the features that the `Chunk` trait exposes. All `Chunk`
/// objects must implement `Sampleable`.
///
/// The generic type `DataType` represents the type of the data returned by
/// sampling, and `N` represents the dimension of the coordinates required to
/// access the data.
///
/// This trait is used because `ChunkIterator`s access the data of the chunk
/// they iterate over. The `ChunkIterator` wants to borrow a dynamic `Chunk`
/// trait object, and the compiler throws errors when checking some features
/// exposed by `Chunk`.
///
/// Instead, the `ChunkIterator` borrows a dynamic `Sampleable` trait object,
/// which only includes features a `ChunkIterator` needs. This circumvents the
/// features in the `Chunk` trait which cause compiler errors.
pub trait Sampleable<DataType, const N: usize> {
    /// Returns the element of data found at the coordinates given by `pos`.
    fn sample(&self, pos: [usize; N]) -> DataType;
}

/// A `Chunk` is a contiguous, `N`-dimensional set of data in memory. When
/// representing terrain, they are usually 2D or 3D blocks that contain height
/// or material data at each coordinate position.
///
/// The generic type `DataType` represents the type of the data returned by
/// sampling, and `N` represents the dimension of the coordinates required to
/// access the data.
///
/// The `Chunk` type is a trait, and must be implemented by a struct type to be
/// used. `Chunk2x64` and `Chunk3x16` are two such implemented types that you
/// can import and use, or you can use the `create_chunk_type` macro to design a
/// chunk with a different size or `DataType`.
///
/// `Chunk`s must have an equal width on all sides, and that width must be some
/// power of two. Ideally you should limit the total size of the chunk. 4096
/// bytes is a gooding starting point, which is a chunk width of 64 in 2D
/// chunks, and 16 in 3D chunks. From there, you increase or decrease to see
/// what performs best for your use case.
pub trait Chunk<DataType, const N: usize>: Sampleable<DataType, N> + Sized {
    /// The width of one side of this `Chunk`. All sides of a `Chunk`
    /// must have equal widths, and this width must be a power of two.
    const WIDTH: usize;

    /// The total size of this `Chunk`. Equal to the area of the `Chunk` for 2D
    /// `Chunk`s, and the volume for 3D `Chunk`s.
    const SIZE: usize;

    /// Creates a new `Chunk` of this type.
    fn new() -> Self;

    /// Converts `N`-dimensional coordinates used to access data from this
    /// `Chunk` into the one-dimensional index used to access the array that
    /// underlies this `Chunk`.
    fn linearize(pos: [usize; N]) -> usize;

    /// Converts one-dimensional coordinates used to access the array that
    /// underlies this `Chunk` data structure, into the the `N`-dimensional
    /// coordinates that the `Chunk` reads.
    fn delinearize(index: usize) -> [usize; N];

    /// Updates a value at a given coordinate position in the chunk.
    fn write(&mut self, pos: [usize; N], val: DataType);

    /// Creates a new iterator for this `Chunk`, allowing you to access every
    /// data element of the `Chunk` sequentially. Every iteration gives you
    /// access to a tuple containing both an `N`-sized array of the coordinates
    /// for the given iteration, and the data value found at those coordinates
    /// for this iteration.
    ///
    /// ```
    /// let chunk = Chunk2x64::new();
    /// for (coords, value) in chunk.iter() {
    ///     println!("We found {} at x: {}, y: {}.", value, coords[0], coords[1]);
    ///     // Ex: "We found 0 at x: 0, y: 0."
    /// }
    /// ```
    fn iter(&self) -> ChunkIterator<'_, Self, DataType, N>;
}

/// The `ChunkIterator` allows you to access every element of data in a `Chunk`
/// sequentially. Every iteration gives you access to a tuple containing both an
/// `N`-sized array of the coordinates for the given iteration, and the data
/// value found at those coordinates for this iteration.
///
/// ```
/// let chunk = Chunk2x64::new();
/// for (coords, value) in chunk.iter() {
///     println!("We found {} at x: {}, y: {}.", value, coords[0], coords[1]);
///     // Ex: "We found 0 at x: 0, y: 0."
/// }
/// ```
///
/// The generic type `ChunkType` is passed as phantom data to the
/// `ChunkIterator`, so that it can access `Chunk` constants specific to the
/// chunk type of the chunk that this iterator is assigned to. `DataType`
/// represents the type of the data returned during iteration, and `N`
/// represents the dimension of the coordinates linked to that data. Note that
/// the data type that the `ChunkIterator` accesses is under the `Sampleable`
/// trait, which exposes a subset of features of the `Chunk` trait. See the docs
/// on `Sampleable` for more information.
pub struct ChunkIterator<'a, ChunkType, DataType, const N: usize> {
    index: usize,
    chunk: &'a dyn Sampleable<DataType, N>,
    chunk_type: PhantomData<ChunkType>,
}

impl<ChunkType, DataType, const N: usize> Iterator for ChunkIterator<'_, ChunkType, DataType, N>
where
    ChunkType: Chunk<DataType, N>,
{
    type Item = ([usize; N], DataType);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= ChunkType::SIZE {
            None
        } else {
            let pos = ChunkType::delinearize(self.index);
            self.index += 1;
            Some((pos, self.chunk.sample(pos)))
        }
    }
}

/// Shorthand functions to create a 2D shape for a `Chunk`  . `Chunk` shapes are
/// defined using the `ndshape` crate, and these functions are presented to a
/// developer using `bevy_terrain` so that they can avoid an extra import for
/// `ndshape` in their code.
#[inline]
pub const fn chunk_shape_2d<const X: usize>() -> ConstPow2Shape2usize<X, X> {
    ConstPow2Shape2usize::<X, X>
}

/// Shorthand functions to create a 3D shape for a `Chunk`  . `Chunk` shapes are
/// defined using the `ndshape` crate, and these functions are presented to a
/// developer using `bevy_terrain` so that they can avoid an extra import for
/// `ndshape` in their code.
#[inline]
pub const fn chunk_shape_3d<const X: usize>() -> ConstPow2Shape3usize<X, X, X> {
    ConstPow2Shape3usize::<X, X, X>
}

/// Shorthand functions to create a 4D shape for a `Chunk`  . `Chunk` shapes are
/// defined using the `ndshape` crate, and these functions are presented to a
/// developer using `bevy_terrain` so that they can avoid an extra import for
/// `ndshape` in their code.
#[inline]
pub const fn chunk_shape_4d<const X: usize>() -> ConstPow2Shape4usize<X, X, X, X> {
    ConstPow2Shape4usize::<X, X, X, X>
}

/// `create_chunk_type` allows developers to easily create new formats and sizes
/// for `Chunk`s. This macro is best demonstrated through example.
///
/// ```
/// // Create a new data type which will represent an individual element in the
/// // chunk
/// struct NewDataType {
///     height: u8,
///     material: u8
/// }
///
/// // All sides of a chunk are powers of two. We define the side length using
/// // the exponent. For example, this chunk's side length will be 2^6 = 64.
/// const EXP: usize = 6;
///
/// // The dimensionality of the chunk--this chunk will be 2D.
/// const DIM: usize = 2;
///
/// // Using our shorthand method, we create a new 2D shape matching the chunk's
/// // width (again, in terms of the exponent).
/// let new_chunk_shape = chunk_shape_2d::<EXP>();
///
/// // Slot the arguments in the right place and we get our new chunk called
/// // NewChunkType.
/// create_chunk_type!(NewChunkType, NewDataType, new_chunk_shape, EXP, DIM);
/// ```
///
/// As you can see, the first argument to the macro will be the name of the new
/// `Chunk` type. The second, the type of your data storage. Third, your chunk
/// shape. Fourth, your width in terms of the exponent over two, and last is the
/// dimensionality.
macro_rules! create_chunk_type {
    ($name: ident, $data_type: ident, $shape: expr, $exp: expr, $dim: expr) => {
        /// This is a macro-generated chunk type. The naming convention for
        /// `Chunk` types that come from `bevy_terrain` is `ChunkAxB`, where `A`
        /// is the dimensionality (2D vs 3D, for example), and `B` is the width
        /// of the chunk for one dimension (all dimensions are equal width). See
        /// the docs for `Chunk` and `create_chunk_type` for more information.
        #[derive(Component)]
        pub struct $name {
            data: [$data_type; Self::SIZE],
        }

        impl $name {
            #[inline]
            fn shape() -> impl Shape<$dim, Coord = usize> {
                $shape
            }
        }

        impl Sampleable<$data_type, $dim> for $name {
            #[inline]
            fn sample(&self, pos: [usize; $dim]) -> $data_type {
                self.data[Self::linearize(pos)]
            }
        }

        impl Chunk<$data_type, $dim> for $name {
            const WIDTH: usize = 2_usize.pow($exp);
            const SIZE: usize = Self::WIDTH.pow($dim);

            fn new() -> Self {
                Self {
                    data: [$data_type::default(); Self::SIZE],
                }
            }

            #[inline]
            fn linearize(pos: [usize; $dim]) -> usize {
                Self::shape().linearize(pos)
            }

            #[inline]
            fn delinearize(index: usize) -> [usize; $dim] {
                Self::shape().delinearize(index)
            }

            #[inline]
            fn write(&mut self, pos: [usize; $dim], val: $data_type) {
                self.data[Self::linearize(pos)] = val;
            }

            fn iter(&self) -> ChunkIterator<'_, Self, $data_type, $dim> {
                ChunkIterator {
                    index: 0,
                    chunk: self,
                    chunk_type: PhantomData,
                }
            }
        }
    };
}

create_chunk_type!(Chunk2x64, u8, chunk_shape_2d::<6>(), 6, 2);
create_chunk_type!(Chunk2x128, u8, chunk_shape_2d::<7>(), 7, 2);
create_chunk_type!(Chunk3x16, u8, chunk_shape_3d::<4>(), 4, 3);
create_chunk_type!(Chunk3x32, u8, chunk_shape_3d::<5>(), 5, 3);
