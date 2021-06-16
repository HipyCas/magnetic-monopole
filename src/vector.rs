use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Copy, Clone)]
pub enum Axis {
  X,
  Y,
  Z,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Vec3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Vec3 {
  pub const ZERO: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
  };

  pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
  }

  pub fn component(&self, axis: Axis) -> f32 {
    match axis {
      Axis::X => self.x,
      Axis::Y => self.y,
      Axis::Z => self.z,
    }
  }

  /*
  pub fn unitary(self) -> Vec3 {
    self / self.length();
  }

  pub fn length(&self) -> f32 {
    ()
  }
  */

  pub fn vectorial_product(&self, v: &Vec3) -> Vec3 {
    Vec3 {
      x: self.y * v.z - self.z * v.y,
      y: self.z * v.x - self.x * v.z,
      z: self.x * v.y - self.y * v.x,
    }
  }

  pub fn cross(e: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {
      x: e.y * v.z - e.z * v.y,
      y: e.z * v.x - e.x * v.z,
      z: e.x * v.y - e.y * v.x,
    }
  }
}

macro_rules! impl_binary_operations {
  // $VectorType is something like `Vec3`
  // $Operation is something like `Add`
  // $op_fn is something like `add`
  // $op_symbol is something like `+`
  ($VectorType:ident $Operation:ident $op_fn:ident $op_symbol:tt) => {
    // Implement a + b where a and b are both of type &VectorType.
    // Lower down we'll implement cases where either a or b - or both
    // - are values by forwarding through to this implementation.
    impl<'a, 'b> $Operation<&'a $VectorType> for &'b $VectorType {
      type Output = $VectorType;
      fn $op_fn(self, other: &'a $VectorType) -> $VectorType {
        $VectorType {
          x: self.x $op_symbol other.x,
          y: self.y $op_symbol other.y,
          z: self.z $op_symbol other.z,
        }
      }
    }

    // Implement a + b for the cases...
    //
    //   a: $VectorType,  b: &$VectorType
    //   a: &$VectorType, b: $VectorType
    //   a: $VectorType, b: $VectorType
    //
    // In each case we forward through to the implementation above.
    impl $Operation<$VectorType> for $VectorType {
      type Output = $VectorType;

      #[inline]
      fn $op_fn(self, other: $VectorType) -> $VectorType {
        &self $op_symbol &other
      }
    }

    impl<'a> $Operation<&'a $VectorType> for $VectorType {
      type Output = $VectorType;

      #[inline]
      fn $op_fn(self, other: &'a $VectorType) -> $VectorType {
        &self $op_symbol other
      }
    }

    impl<'a> $Operation<$VectorType> for &'a $VectorType {
      type Output = $VectorType;

      #[inline]
      fn $op_fn(self, other: $VectorType) -> $VectorType {
        self $op_symbol &other
      }
    }

    // Implement a + b where a is type &$VectorType and b is type f32
    impl<'a> $Operation<f32> for &'a $VectorType {
      type Output = $VectorType;

      fn $op_fn(self, other: f32) -> $VectorType {
        $VectorType {
          x: self.x $op_symbol other,
          y: self.y $op_symbol other,
          z: self.z $op_symbol other
        }
      }
    }

    // Implement a + b where...
    //
    // a is $VectorType and b is f32
    // a is f32 and b is $VectorType
    // a is f32 and b is &$VectorType
    //
    // In each case we forward the logic to the implementation
    // above.
    impl $Operation<f32> for $VectorType {
      type Output = $VectorType;

      #[inline]
      fn $op_fn(self, other: f32) -> $VectorType {
        &self $op_symbol other
      }
    }

    impl $Operation<$VectorType> for f32 {
      type Output = $VectorType;

      #[inline]
      fn $op_fn(self, other: $VectorType) -> $VectorType {
        &other $op_symbol self
      }
    }

    impl<'a> $Operation<&'a $VectorType> for f32 {
      type Output = $VectorType;

      #[inline]
      fn $op_fn(self, other: &'a $VectorType) -> $VectorType {
        other $op_symbol self
      }
    }
  };
}

macro_rules! impl_unary_operations {
  // $VectorType is something like `Vec3`
  // $Operation is something like `Neg`
  // $op_fn is something like `neg`
  // $op_symbol is something like `-`
  ($VectorType:ident $Operation:ident $op_fn:ident $op_symbol:tt) => {

    // Implement the unary operator for references
    impl<'a> $Operation for &'a $VectorType {
      type Output = $VectorType;

      fn $op_fn(self) -> Vec3 {
        $VectorType {
          x: $op_symbol self.x,
          y: $op_symbol self.y,
          z: $op_symbol self.z,
        }
      }
    }

    // Have the operator on values forward through to the implementation
    // above
    impl $Operation for $VectorType {
      type Output = $VectorType;

      #[inline]
      fn $op_fn(self) -> Vec3 {
        $op_symbol &self
      }
    }
  };
}

// Implement add-assignment operators like a += b where a and
// b is either &Vec3 or Vec3 (in this case a is always of type
// &mut Vec3).
macro_rules! impl_op_assign {
  // $VectorType is something like `Vec3`
  // $OperationAssign is something like `AddAssign`
  // $op_fn is something like `add_assign`
  // $op_symbol is something like `+=`
  ($VectorType:ident $OperationAssign:ident $op_fn:ident $op_symbol:tt) => {
    // Implement $OperationAssign for RHS &Vec3
    impl<'a> $OperationAssign<&'a $VectorType> for $VectorType {
      fn $op_fn(&mut self, other: &'a $VectorType) {
        *self = $VectorType {
          x: self.x $op_symbol other.x,
          y: self.y $op_symbol other.y,
          z: self.z $op_symbol other.z,
        };
      }
    }

    // Implement $OperationAssign for RHS Vec3 by forwarding through to the
    // implementation above
    impl $OperationAssign for $VectorType {
      #[inline]
      fn $op_fn(&mut self, other: $VectorType) {
        *self = *self $op_symbol &other
      }
    }
  };
}

impl_binary_operations!(Vec3 Add add +);
impl_op_assign!(Vec3 AddAssign add_assign +);

impl_binary_operations!(Vec3 Sub sub -);
impl_op_assign!(Vec3 SubAssign sub_assign -);
impl_unary_operations!(Vec3 Neg neg -);

impl_binary_operations!(Vec3 Mul mul *);
impl_op_assign!(Vec3 MulAssign mul_assign *);

impl_binary_operations!(Vec3 Div div /);
impl_op_assign!(Vec3 DivAssign div_assign /);
