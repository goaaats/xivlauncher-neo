pub(crate) mod hex_slice;
pub(crate) mod locale;
#[allow(dead_code)]
pub(crate) mod time;
pub(crate) mod hash;

#[macro_export]
macro_rules! either {
  ($test:expr => $true_expr:expr; $false_expr:expr) => {
    if $test {
      $true_expr
    } else {
      $false_expr
    }
  };
}

