// E - ZeroOne
// Desmond Germans, 2020

/// Adds standardized zero element to a type.
/// 
/// This allows for numerical generic
/// operations to span integer and floating point types, as well as clear
/// vectors, arrays, matrices, volumes, etc.
pub trait Zero {
    /// Returns the zero element for this type.
    /// # Examples
    /// ```
    /// let origin = u32::zero();
    /// assert_eq!(0,origin);
    /// ```
    fn zero() -> Self;
}

impl Zero for bool { fn zero() -> Self { false } }
impl Zero for u8 { fn zero() -> Self { 0 } }
impl Zero for i8 { fn zero() -> Self { 0 } }
impl Zero for u16 { fn zero() -> Self { 0 } }
impl Zero for i16 { fn zero() -> Self { 0 } }
impl Zero for u32 { fn zero() -> Self { 0 } }
impl Zero for i32 { fn zero() -> Self { 0 } }
impl Zero for u64 { fn zero() -> Self { 0 } }
impl Zero for i64 { fn zero() -> Self { 0 } }
impl Zero for usize { fn zero() -> Self { 0 } }
impl Zero for isize { fn zero() -> Self { 0 } }
impl Zero for f32 { fn zero() -> Self { 0.0 } }
impl Zero for f64 { fn zero() -> Self { 0.0 } }

/// Adds standardized one or unit element to a type.
/// 
/// This allows for numerical
/// generic operations to span integer and floating point types, as well as
/// specify unit matrices.
pub trait One {
    /// Returns the one element for this type.
    /// # Examples
    /// ```
    /// let unit = u32::one();
    /// assert_eq!(1,unit);
    /// ```
    fn one() -> Self;
}

impl One for bool { fn one() -> Self { true } }
impl One for u8 { fn one() -> Self { 1 } }
impl One for i8 { fn one() -> Self { 1 } }
impl One for u16 { fn one() -> Self { 1 } }
impl One for i16 { fn one() -> Self { 1 } }
impl One for u32 { fn one() -> Self { 1 } }
impl One for i32 { fn one() -> Self { 1 } }
impl One for u64 { fn one() -> Self { 1 } }
impl One for i64 { fn one() -> Self { 1 } }
impl One for usize { fn one() -> Self { 1 } }
impl One for isize { fn one() -> Self { 1 } }
impl One for f32 { fn one() -> Self { 1.0 } }
impl One for f64 { fn one() -> Self { 1.0 } }
