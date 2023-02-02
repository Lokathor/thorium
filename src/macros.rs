#![allow(unused_imports)]

macro_rules! impl_bit_ops {
  ($type_name:ident) => {
    impl core::ops::BitAnd for $type_name {
      type Output = Self;
      #[inline]
      #[must_use]
      fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0.bitand(rhs.0))
      }
    }
    impl core::ops::BitOr for $type_name {
      type Output = Self;
      #[inline]
      #[must_use]
      fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0.bitor(rhs.0))
      }
    }
    impl core::ops::BitXor for $type_name {
      type Output = Self;
      #[inline]
      #[must_use]
      fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0.bitxor(rhs.0))
      }
    }
    impl core::ops::Not for $type_name {
      type Output = Self;
      #[inline]
      #[must_use]
      fn not(self) -> Self::Output {
        Self(self.0.not())
      }
    }
    impl core::ops::BitAndAssign for $type_name {
      #[inline]
      fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
      }
    }
    impl core::ops::BitOrAssign for $type_name {
      #[inline]
      fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
      }
    }
    impl core::ops::BitXorAssign for $type_name {
      #[inline]
      fn bitxor_assign(&mut self, rhs: Self) {
        self.0.bitxor_assign(rhs.0)
      }
    }
  };
}
pub(crate) use impl_bit_ops;
