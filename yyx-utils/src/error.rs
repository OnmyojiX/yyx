#[doc(hidden)]
#[macro_export]
macro_rules! impl_err_from_impl {
  (
    $e:ty,
    ($from_e:path => |$binding:ident| $from:expr)
  ) => {
    impl From<$from_e> for $e {
      fn from($binding: $from_e) -> Self {
        $from
      }
    }
  };

  (
    $e:ty,
    ($from_e:path => $from:expr)
  ) => {
    impl From<$from_e> for $e {
      fn from(_e: $from_e) -> Self {
        $from
      }
    }
  };
}

#[macro_export]
macro_rules! impl_err_from {
  (
    $e:ty [
      $(
        $tt:tt
      ),*
    ]
  ) => {
    $(
      impl_err_from_impl!($e, $tt);
    )*
  };
}

#[test]
fn test_impl_err_from() {
  #[derive(Debug, PartialEq)]
  enum E {
    E1,
    E2(E2),
  }

  #[derive(Debug, PartialEq)]
  struct E1;

  #[derive(Debug, PartialEq)]
  struct E2;

  impl_err_from! {
    E [
      (E1 => E::E1),
      (E2 => |e| E::E2(e))
    ]
  }

  assert_eq!(E::from(E1), E::E1);
  assert_eq!(E::from(E2), E::E2(E2));
}
