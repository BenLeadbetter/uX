//! #uX - non-standard-width integers types
//!
//! When non-standard-width integers is required in an applications, the norm is to use a larger container and make sure the value is within range after manipulation. uX aims to take care of this once and for all by:
//! 
//! - Providing `u2`-`u63` and `i2`-`i63` types that should behave as similar as possible to the built in rust types
//!     - The methods of the defined types are the same as for the built in types (far from all is implemented at this point but fill out an issue or create a PR if something essential for you is missing)
//!     - Overflow will panic in debug and wrap in release.
//! - When `i128` and `u128` is stabilized this crate will also support `u65-u127` and `i65-i127`
//! - All possible lossless conversions is possible by using `From`.
//! - When `TryFrom` is stabilized fallible conversions will also be supported.


#![cfg_attr(not(feature="std"), no_std)]


mod lib {
    pub mod core {
        #[cfg(feature="std")]
        pub use std::*;
        #[cfg(not(feature="std"))]
        pub use core::*;
    }
}

mod conversion;

use lib::core::ops::{
    Shr,
    ShrAssign,
    Shl,
    ShlAssign,
    BitOr,
    BitOrAssign,
};

use lib::core::hash::{
    Hash,
    Hasher,
};

use lib::core::cmp::{
    Ordering,
    Ord,
    PartialOrd,
};

use lib::core::fmt::{
    Display,
    Formatter,
    UpperHex,
    LowerHex,
    Octal,
    Binary,
};

macro_rules! define_unsigned {
    ($name:ident, $bits:expr, $type:ident) => {
        #[allow(non_camel_case_types)]
        #[derive(Default, Clone, Copy, Debug)]
        pub struct $name($type);

        impl $name {
            pub const MAX: Self = $name(((1 as $type) << $bits) -1 );
            pub const MIN: Self = $name(0);

            fn mask(self) -> Self {
                $name(self.0 & ( ((1 as $type) << $bits).overflowing_sub(1).0))
            }
        }
        
        implement_common!($name, $bits, $type);
        
    }
}

macro_rules! define_signed {
    ($name:ident, $bits:expr, $type:ident) => {
        #[allow(non_camel_case_types)]
        #[derive(Default, Clone, Copy, Debug)]
        pub struct $name($type);

        impl $name {
            pub const MAX: Self = $name(((1 as $type) << ($bits - 1)) - 1);
            pub const MIN: Self = $name(-((1 as $type) << ($bits - 1)));

            fn mask(self) -> Self {
                if ( self.0 & (1<<($bits-1)) ) == 0 {
                    $name(self.0 & ( ((1 as $type) << $bits).overflowing_sub(1).0))
                } else {
                    $name(self.0 | !( ((1 as $type) << $bits).overflowing_sub(1).0))
                }
            }
        }
        
        implement_common!($name, $bits, $type);
        
    }
}

macro_rules! implement_common {
    ($name:ident, $bits:expr, $type:ident) => {
        impl $name {
            /// Returns the smallest value that can be represented by this integer type.
            pub fn min_value() -> $name {
                $name::MIN
            }
            /// Returns the largest value that can be represented by this integer type.
            pub fn max_value() -> $name {
                $name::MAX
            }

            /// Crates a new variable
            ///
            /// # Panic
            /// This function will panic if `value` is not representable by this type
            pub fn new(value: $type) -> $name {
                assert!(value <= $name::MAX.0 && value >= $name::MIN.0);
                $name(value)
            }

            /// Wrapping (modular) subtraction. Computes `self - other`,
            /// wrapping around at the boundary of the type.
            ///
            /// # Examples
            ///
            /// Basic usage:
            ///
            /// ```
            /// use ux::*;
            ///
            /// assert_eq!(i5::MIN.wrapping_sub(i5::new(1)), i5::MAX);
            ///
            /// assert_eq!(i5::new(-10).wrapping_sub(i5::new(5)), i5::new(-15));
            /// assert_eq!(i5::new(-15).wrapping_sub(i5::new(5)), i5::new(12));
            /// ```
            pub fn wrapping_sub(self, rhs: Self) -> Self {
                $name(self.0.wrapping_sub(rhs.0)).mask()
            }
            
            /// Wrapping (modular) addition. Computes `self + other`,
            /// wrapping around at the boundary of the type.
            ///
            /// # Examples
            ///
            /// Basic usage:
            ///
            /// ```
            /// use ux::*;
            ///
            /// assert_eq!(i5::MAX.wrapping_add(i5::new(1)), i5::MIN);
            ///
            /// assert_eq!(i5::new(10).wrapping_add(i5::new(5)), i5::new(15));
            /// assert_eq!(i5::new(15).wrapping_add(i5::new(5)), i5::new(-12));
            /// ```
            pub fn wrapping_add(self, rhs: Self) -> Self {
                $name(self.0.wrapping_add(rhs.0)).mask()
            }
            
        }

        
        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.mask().0 == other.mask().0
            }
        }
        
        impl Eq for $name {}

        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &$name) -> Option<Ordering> {
                self.mask().0.partial_cmp(&other.mask().0)
            }
        }

        impl Ord for $name {
            fn cmp(&self, other: &$name) -> Ordering {
                self.mask().0.cmp(&other.mask().0)
            }
        }

        impl Hash for $name {
            fn hash<H: Hasher>(&self, h: &mut H) {
                self.mask().0.hash(h)
            }
        }
        
        // Implement formating functions
        impl Display for $name {
            fn fmt(&self, f: &mut Formatter) -> Result<(), lib::core::fmt::Error> {
                let &$name(ref value) = self;
                <$type as Display>::fmt(value, f)
            }
        }
        impl UpperHex for $name {
            fn fmt(&self, f: &mut Formatter) -> Result<(), lib::core::fmt::Error> {
                let &$name(ref value) = self;
                <$type as UpperHex>::fmt(value, f)
            }
        }
        impl LowerHex for $name {
            fn fmt(&self, f: &mut Formatter) -> Result<(), lib::core::fmt::Error> {
                let &$name(ref value) = self;
                <$type as LowerHex>::fmt(value, f)
            }
        }
        impl Octal for $name {
            fn fmt(&self, f: &mut Formatter) -> Result<(), lib::core::fmt::Error> {
                let &$name(ref value) = self;
                <$type as Octal>::fmt(value, f)
            }
        }
        impl Binary for $name {
            fn fmt(&self, f: &mut Formatter) -> Result<(), lib::core::fmt::Error> {
                let &$name(ref value) = self;
                <$type as Binary>::fmt(value, f)
            }
        }

        impl<T> Shr<T> for $name where $type: Shr<T, Output=$type>{
            type Output = $name;

            fn shr(self, rhs: T) -> $name {
                $name(self.mask().0.shr(rhs))
            }
        }

        impl<T> Shl<T> for $name where $type: Shl<T, Output=$type> {
            type Output = $name;

            fn shl(self, rhs: T) -> $name {
                $name(self.mask().0.shl(rhs))
            }
        }

        impl<T> ShrAssign<T> for $name where $type: ShrAssign<T> {
            fn shr_assign(&mut self, rhs: T) {
                *self = self.mask();
                self.0.shr_assign(rhs);
            }
        }

        impl<T> ShlAssign<T> for $name where $type: ShlAssign<T> {
            fn shl_assign(&mut self, rhs: T) {
                *self = self.mask();
                self.0.shl_assign(rhs);
            }
        }

        impl BitOr<$name> for $name {
            type Output = $name;
            
            fn bitor(self, rhs: $name) -> Self::Output {
                $name(self.mask().0.bitor(rhs.mask().0))
            }
        }

        impl<'a> BitOr<&'a $name> for $name {
            type Output = <$name as BitOr<$name>>::Output;
            
            fn bitor(self, rhs: &'a $name) -> Self::Output {
                $name(self.mask().0.bitor(rhs.mask().0))
            }
        }

        impl<'a> BitOr<$name> for &'a $name {
            type Output = <$name as BitOr<$name>>::Output;
            
            fn bitor(self, rhs: $name) -> Self::Output {
                $name(self.mask().0.bitor(rhs.mask().0))
            }
        }

        impl<'a> BitOr<&'a $name> for &'a $name {
            type Output = <$name as BitOr<$name>>::Output;
            
            fn bitor(self, rhs: &'a $name) -> Self::Output {
                $name(self.mask().0.bitor(rhs.mask().0))
            }
        }

        impl BitOrAssign<$name> for $name {
            fn bitor_assign(&mut self, other: $name) {
                *self = self.mask();
                self.0.bitor_assign(other.mask().0)
            }
        }

        

        
    };
}


define_unsigned!(u2, 2, u8);
define_unsigned!(u3, 3, u8);
define_unsigned!(u4, 4, u8);
define_unsigned!(u5, 5, u8);
define_unsigned!(u6, 6, u8);
define_unsigned!(u7, 7, u8);

define_unsigned!(u9, 9, u16);
define_unsigned!(u10, 10, u16);
define_unsigned!(u11, 11, u16);
define_unsigned!(u12, 12, u16);
define_unsigned!(u13, 13, u16);
define_unsigned!(u14, 14, u16);
define_unsigned!(u15, 15, u16);

define_unsigned!(u17, 17, u32);
define_unsigned!(u18, 18, u32);
define_unsigned!(u19, 19, u32);
define_unsigned!(u20, 20, u32);
define_unsigned!(u21, 21, u32);
define_unsigned!(u22, 22, u32);
define_unsigned!(u23, 23, u32);
define_unsigned!(u24, 24, u32);

define_unsigned!(u25, 25, u32);
define_unsigned!(u26, 26, u32);
define_unsigned!(u27, 27, u32);
define_unsigned!(u28, 28, u32);
define_unsigned!(u29, 29, u32);
define_unsigned!(u30, 30, u32);
define_unsigned!(u31, 31, u32);

define_unsigned!(u33, 33, u64);
define_unsigned!(u34, 34, u64);
define_unsigned!(u35, 35, u64);
define_unsigned!(u36, 36, u64);
define_unsigned!(u37, 37, u64);
define_unsigned!(u38, 38, u64);
define_unsigned!(u39, 39, u64);
define_unsigned!(u40, 40, u64);

define_unsigned!(u41, 41, u64);
define_unsigned!(u42, 42, u64);
define_unsigned!(u43, 43, u64);
define_unsigned!(u44, 44, u64);
define_unsigned!(u45, 45, u64);
define_unsigned!(u46, 46, u64);
define_unsigned!(u47, 47, u64);
define_unsigned!(u48, 48, u64);

define_unsigned!(u49, 49, u64);
define_unsigned!(u50, 50, u64);
define_unsigned!(u51, 51, u64);
define_unsigned!(u52, 52, u64);
define_unsigned!(u53, 53, u64);
define_unsigned!(u54, 54, u64);
define_unsigned!(u55, 55, u64);
define_unsigned!(u56, 56, u64);

define_unsigned!(u57, 57, u64);
define_unsigned!(u58, 58, u64);
define_unsigned!(u59, 59, u64);
define_unsigned!(u60, 60, u64);
define_unsigned!(u61, 61, u64);
define_unsigned!(u62, 62, u64);
define_unsigned!(u63, 63, u64);


define_signed!(i2, 2, i8);
define_signed!(i3, 3, i8);
define_signed!(i4, 4, i8);
define_signed!(i5, 5, i8);
define_signed!(i6, 6, i8);
define_signed!(i7, 7, i8);
               
define_signed!(i9, 9, i16);
define_signed!(i10, 10, i16);
define_signed!(i11, 11, i16);
define_signed!(i12, 12, i16);
define_signed!(i13, 13, i16);
define_signed!(i14, 14, i16);
define_signed!(i15, 15, i16);
                        
define_signed!(i17, 17, i32);
define_signed!(i18, 18, i32);
define_signed!(i19, 19, i32);
define_signed!(i20, 20, i32);
define_signed!(i21, 21, i32);
define_signed!(i22, 22, i32);
define_signed!(i23, 23, i32);
define_signed!(i24, 24, i32);
                        
define_signed!(i25, 25, i32);
define_signed!(i26, 26, i32);
define_signed!(i27, 27, i32);
define_signed!(i28, 28, i32);
define_signed!(i29, 29, i32);
define_signed!(i30, 30, i32);
define_signed!(i31, 31, i32);
                        
define_signed!(i33, 33, i64);
define_signed!(i34, 34, i64);
define_signed!(i35, 35, i64);
define_signed!(i36, 36, i64);
define_signed!(i37, 37, i64);
define_signed!(i38, 38, i64);
define_signed!(i39, 39, i64);
define_signed!(i40, 40, i64);
                        
define_signed!(i41, 41, i64);
define_signed!(i42, 42, i64);
define_signed!(i43, 43, i64);
define_signed!(i44, 44, i64);
define_signed!(i45, 45, i64);
define_signed!(i46, 46, i64);
define_signed!(i47, 47, i64);
define_signed!(i48, 48, i64);
                        
define_signed!(i49, 49, i64);
define_signed!(i50, 50, i64);
define_signed!(i51, 51, i64);
define_signed!(i52, 52, i64);
define_signed!(i53, 53, i64);
define_signed!(i54, 54, i64);
define_signed!(i55, 55, i64);
define_signed!(i56, 56, i64);
                        
define_signed!(i57, 57, i64);
define_signed!(i58, 58, i64);
define_signed!(i59, 59, i64);
define_signed!(i60, 60, i64);
define_signed!(i61, 61, i64);
define_signed!(i62, 62, i64);
define_signed!(i63, 63, i64);

            
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_masking() {
        assert_eq!(u4(0b11000110).mask().0, 0b00000110);
        assert_eq!(u4(0b00001000).mask().0, 0b00001000);
        assert_eq!(u4(0b00001110).mask().0, 0b00001110);
        
        assert_eq!(i4(0b11000110u8 as i8).mask().0, 0b00000110u8 as i8);
        assert_eq!(i4(0b00001000u8 as i8).mask().0, 0b11111000u8 as i8);
        assert_eq!(i4(0b00001110u8 as i8).mask().0, 0b11111110u8 as i8);
        
    }
    
    #[test]
    fn min_max_values() {
        assert_eq!(u2::MAX, u2(3));
        assert_eq!(u3::MAX, u3(7));
        assert_eq!(u7::MAX, u7(127));
        assert_eq!(u9::MAX, u9(511));

        
        assert_eq!(i2::MAX, i2(1));
        assert_eq!(i3::MAX, i3(3));
        assert_eq!(i7::MAX, i7(63));
        assert_eq!(i9::MAX, i9(255));

        
        assert_eq!(u2::MIN, u2(0));
        assert_eq!(u3::MIN, u3(0));
        assert_eq!(u7::MIN, u7(0));
        assert_eq!(u9::MIN, u9(0));

        
        assert_eq!(i2::MIN, i2(-2));
        assert_eq!(i3::MIN, i3(-4));
        assert_eq!(i7::MIN, i7(-64));
        assert_eq!(i9::MIN, i9(-256));

        
    }

    #[test]
    fn test_wrapping_add() {
        assert_eq!(u5::MAX.wrapping_add(u5(1)), u5(0));
        assert_eq!(u5::MAX.wrapping_add(u5(4)), u5(3));
        
        assert_eq!(i7::MAX.wrapping_add(i7(1)), i7::MIN);
        assert_eq!(i7::MAX.wrapping_add(i7(4)), i7(-61));
    }

    #[test]
    fn test_shr() {
        assert_eq!(u5(8) >> 1usize, u5(4));
        assert_eq!(u5(8) >> 1u8, u5(4));
        assert_eq!(u5(8) >> 1u16, u5(4));
        assert_eq!(u5(8) >> 1u32, u5(4));
        assert_eq!(u5(8) >> 1u64, u5(4));
        assert_eq!(u5(8) >> 1isize, u5(4));
        assert_eq!(u5(8) >> 1i8, u5(4));
        assert_eq!(u5(8) >> 1i16, u5(4));
        assert_eq!(u5(8) >> 1i32, u5(4));
        assert_eq!(u5(8) >> 1i64, u5(4));
        
        assert_eq!(u5::MAX >> 4, u5(1));
        
        assert_eq!(i7(-1) >> 5, i7(-1));
    }
    
    #[test]
    fn test_shl() {
        assert_eq!(u5(16) << 1usize, u5(32));
        assert_eq!(u5(16) << 1u8, u5(32));
        assert_eq!(u5(16) << 1u16, u5(32));
        assert_eq!(u5(16) << 1u32, u5(32));
        assert_eq!(u5(16) << 1u64, u5(32));
        assert_eq!(u5(16) << 1isize, u5(32));
        assert_eq!(u5(16) << 1i8, u5(32));
        assert_eq!(u5(16) << 1i16, u5(32));
        assert_eq!(u5(16) << 1i32, u5(32));
        assert_eq!(u5(16) << 1i64, u5(32));

        assert_eq!(u5::MAX << 4, u5(16));
        
        assert_eq!(i5(16) << 1, i5(0));
        assert_eq!(i7(1) << 3, i7(8));
    }

    #[test]
    fn test_shr_assign() {
        let mut x = u10(512);
        x >>= 1usize;
        assert_eq!(x, u10(256));
        x >>= 1isize;
        assert_eq!(x, u10(128));
        x >>= 1u8;
        assert_eq!(x, u10(64));
        x >>= 1i8;
        assert_eq!(x, u10(32));
        x >>= 2u64;
        assert_eq!(x, u10(8));
        x >>= 3i32;
        assert_eq!(x, u10(1));
    }
    
    #[test]
    fn test_shl_assign() {
        let mut x = u9(1);
        x <<= 3i32;
        assert_eq!(x, u9(8));
        x <<= 2u64;
        assert_eq!(x, u9(32));
        x <<= 1usize;
        assert_eq!(x, u9(64));
        x <<= 1isize;
        assert_eq!(x, u9(128));
        x <<= 1u8;
        assert_eq!(x, u9(256));
    }

    #[test]
    fn test_bitor() {
        assert_eq!(u9(1) | u9(8), u9(9));
        assert_eq!(&u9(1) | u9(8), u9(9));
        assert_eq!(u9(1) | &u9(8), u9(9));
        assert_eq!(&u9(1) | &u9(8), u9(9));
    }
    
    #[test]
    fn test_bitor_assign() {
        let mut x = u12(4);
        x |= u12(1);
        assert_eq!(x, u12(5));
        x |= u12(128);
        assert_eq!(x, u12(133));
        x = u12(1);
        x |= u12(127);
        assert_eq!(x, u12(127));
    }
    


}
