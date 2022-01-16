#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(has_const_float_classify, feature(const_float_classify))]
#![allow(uncommon_codepoints)]
#![warn(rust_2018_idioms, unreachable_pub)]
#![forbid(unsafe_code)]

use core::{
	cmp::{Eq, Ord, Ordering},
	fmt::{self, Debug, Display, Formatter},
	hash::{Hash, Hasher},
	ops::{
		Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign
	},
	str::FromStr
};
use paste::paste;
#[cfg(feature = "serde")]
use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
#[cfg(feature = "std")]
use thiserror::Error;

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "std", derive(Error))]
#[cfg_attr(feature = "std", error("non-finite number"))]
pub struct NonFinite;

macro_rules! unit {
	(pub const struct $name:ident($inner:ident);) => {
		unit!(@internal [const]; $name($inner););
	};

	(pub struct $name:ident($inner:ident);) => {
		unit!(@internal []; $name($inner););
	};

	(@internal [$($const:tt)?]; $name:ident($inner:ident);) => {
		paste! {
			mod [<unit_ $name>] {
				use super::NonFinite;

				#[derive(Clone, Copy, PartialEq, PartialOrd)]
				#[allow(non_camel_case_types)]
				pub struct $name($inner);

				impl $name {
					/// This is a helper method for creating this type. Take a look at the [`Unit`]
					/// trait for a more idiomatic way to create this type.
					///
					/// **Note:** This method is only `const` when using a nightly Rust compiler.
					///
					///  [`Unit`]: super::Unit
					pub $($const)? fn try_new(inner: $inner) -> Result<Self, NonFinite> {
						if !inner.is_finite() {
							return Err(NonFinite);
						}
						Ok(Self(inner))
					}

					/// This is a helper method for creating this type. Take a look at the [`Unit`]
					/// trait for a more idiomatic way to create this type.
					///
					/// **Note:** This method is only `const` when using a nightly Rust compiler.
					///
					/// ### Panics
					///
					/// This method panics if the inner value is non-finite.
					///
					///  [`Unit`]: super::Unit
					pub $($const)? fn new(inner: $inner) -> Self {
						match Self::try_new(inner) {
							Ok(unit) => unit,
							Err(NonFinite) => panic!(concat!(
								stringify!($name),
								" only supports finite numbers"
							))
						}
					}

					/// Return the raw value.
					pub const fn raw_value(self) -> $inner {
						self.0
					}
				}
			}
			pub use [<unit_ $name>]::$name;
		}

		impl Default for $name
		where
			$inner: Default
		{
			fn default() -> Self {
				Self::new(Default::default())
			}
		}

		#[cfg(feature = "std")]
		impl $name {
			/// Returns the nearest integer to `self`. Round half-way cases away
			/// from `0.0`.
			#[must_use = "method returns a new number and does not mutate the original value"]
			pub fn round(self) -> Self {
				Self::new(self.raw_value().round())
			}

			/// Returns the absolute value of `self`.
			#[must_use = "method returns a new number and does not mutate the original value"]
			pub fn abs(self) -> Self {
				Self::new(self.raw_value().abs())
			}

			/// Computes the four quadrant arctangent of `self` and `other`
			/// in radians.
			#[must_use = "method returns a new number and does not mutate the original value"]
			pub fn atan2(self, other: Self) -> f32 {
				self.raw_value().atan2(other.raw_value())
			}
		}

		impl Eq for $name {}

		impl Ord for $name {
			fn cmp(&self, other: &Self) -> Ordering {
				// unwrap: Self will never store non-finite values, so all values
				// should be comparable
				self.partial_cmp(other).unwrap()
			}
		}

		impl Hash for $name {
			fn hash<H: Hasher>(&self, state: &mut H) {
				Hash::hash::<H>(&self.raw_value().to_bits(), state);
			}
		}

		impl Neg for $name {
			type Output = Self;
			fn neg(self) -> Self {
				Self::new(-self.raw_value())
			}
		}

		impl Add for $name {
			type Output = Self;
			fn add(self, rhs: Self) -> Self {
				Self::new(self.raw_value() + rhs.raw_value())
			}
		}

		impl AddAssign for $name {
			fn add_assign(&mut self, rhs: Self) {
				*self = *self + rhs;
			}
		}

		impl Sub for $name {
			type Output = Self;
			fn sub(self, rhs: Self) -> Self {
				Self::new(self.raw_value() - rhs.raw_value())
			}
		}


		impl SubAssign for $name {
			fn sub_assign(&mut self, rhs: Self) {
				*self = *self - rhs;
			}
		}

		impl Mul<$inner> for $name {
			type Output = Self;
			fn mul(self, rhs: $inner) -> Self {
				Self::new(self.raw_value() * rhs)
			}
		}

		impl Mul<$name> for $inner {
			type Output = $name;
			fn mul(self, rhs: $name) -> $name {
				$name::new(self * rhs.raw_value())
			}
		}

		impl MulAssign<$inner> for $name {
			fn mul_assign(&mut self, rhs: $inner) {
				*self = *self * rhs;
			}
		}

		impl Div<$inner> for $name {
			type Output = Self;
			fn div(self, rhs: $inner) -> Self {
				if !rhs.is_finite() {
					panic!("Division through non-finite number");
				}
				if rhs == 0.0 {
					panic!("Division through zero");
				}
				Self::new(self.raw_value() / rhs)
			}
		}

		impl Div<$name> for $name {
			type Output = $inner;
			fn div(self, rhs: $name) -> $inner {
				self.raw_value() / rhs.raw_value()
			}
		}

		impl DivAssign<$inner> for $name {
			fn div_assign(&mut self, rhs: $inner) {
				*self = *self / rhs;
			}
		}

		impl Rem for $name {
			type Output = Self;

			fn rem(self, rhs: Self) -> Self {
				self % rhs.raw_value()
			}
		}

		impl Rem<$inner> for $name {
			type Output = Self;

			fn rem(self, rhs: $inner) -> Self {
				Self::new(self.raw_value() % rhs)
			}
		}

		impl FromStr for $name {
			type Err = <$inner as FromStr>::Err;
			fn from_str(s: &str) -> Result<Self, Self::Err> {
				Ok(Self::new(s.parse()?))
			}
		}

		impl Display for $name {
			fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
				Display::fmt(&self.raw_value(), f)
			}
		}

		impl Debug for $name {
			fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
				Debug::fmt(&self.raw_value(), f)?;
				f.write_str(stringify!($name))
			}
		}

		#[cfg(feature = "serde")]
		impl<'de> Deserialize<'de> for $name {
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
			where
				D: Deserializer<'de>
			{
				$inner::deserialize(deserializer).and_then(|inner| {
					Self::try_new(inner).map_err(|err| D::Error::custom(err))
				})
			}
		}

		#[cfg(feature = "serde")]
		impl Serialize for $name {
			fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
			where
				S: Serializer
			{
				self.raw_value().serialize(serializer)
			}
		}
	};
}

macro_rules! square_unit {
	(pub const struct $name:ident($inner:ident) = $base:ident^2;) => {
		unit!(@internal [const]; $name($inner););
		square_unit!(@internal $name($inner) = $base^2);
	};

	(pub struct $name:ident($inner:ident) = $base:ident^2;) => {
		unit!(@internal []; $name($inner););
		square_unit!(@internal $name($inner) = $base^2);
	};

	(@internal $name:ident($inner:ident) = $base:ident^2) => {
		#[cfg(feature = "std")]
		impl $name {
			pub fn sqrt(self) -> $base {
				$base::new(self.raw_value().sqrt())
			}
		}

		impl Mul<$base> for $base {
			type Output = $name;
			fn mul(self, rhs: $base) -> $name {
				$name::new(self.raw_value() * rhs.raw_value())
			}
		}

		impl Div<$base> for $name {
			type Output = $base;
			fn div(self, rhs: $base) -> $base {
				$base::new(self.raw_value() / rhs.raw_value())
			}
		}
	};
}

#[cfg(not(has_const_float_classify))]
unit! {
	pub struct mm(f32);
}

#[cfg(has_const_float_classify)]
unit! {
	pub const struct mm(f32);
}

#[cfg(not(has_const_float_classify))]
square_unit! {
	pub struct mm2(f32) = mm^2;
}

#[cfg(has_const_float_classify)]
square_unit! {
	pub const struct mm2(f32) = mm^2;
}

macro_rules! unit_trait {
	(pub trait $ident:ident {
		$(fn $name:ident(self) -> $unit:ident $(= self * $convert:literal)? ;)*
	}) => {
		paste! {
			mod [<private_ $ident:lower>] {
				pub trait Sealed {}
				impl Sealed for f32 {}
			}

			pub trait $ident: [<private_ $ident:lower>]::Sealed {
				$(fn $name(self) -> $unit;)*
			}

			impl $ident for f32 {
				$(unit_trait!(@internal fn $name(self) -> $unit $(= self * $convert)?);)*
			}
		}
	};

	(@internal fn $name:ident(self) -> $unit:ident) => {
		fn $name(self) -> $unit {
			$unit::new(self)
		}
	};

	(@internal fn $name:ident(self) -> $unit:ident = self * $convert:literal) => {
		fn $name(self) -> $unit {
			$unit::new(self * $convert)
		}
	};
}

unit_trait! {
	pub trait Unit {
		fn nm(self) -> mm = self * 1e-6;
		fn nm2(self) -> mm2 = self * 1e-12;

		fn µm(self) -> mm = self * 1e-3;
		fn µm2(self) -> mm2 = self * 1e-6;

		fn mm(self) -> mm;
		fn mm2(self) -> mm2;

		fn cm(self) -> mm = self * 1e1;
		fn cm2(self) -> mm2 = self * 1e2;

		fn dm(self) -> mm = self * 1e2;
		fn dm2(self) -> mm2 = self * 1e4;

		fn m(self) -> mm = self * 1e3;
		fn m2(self) -> mm2 = self * 1e6;

		fn km(self) -> mm = self * 1e6;
		fn km2(self) -> mm2 = self * 1e12;

		fn inch(self) -> mm = self * 25.4;
		fn inch2(self) -> mm2 = self * 645.16;
	}
}
