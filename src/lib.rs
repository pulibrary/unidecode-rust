use magnus::{function, Error, Ruby};
use unidecode::unidecode;

fn remove_formatting(a: &str) -> String {
  unidecode(a)
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
  // ruby.define_global_function("remove_formatting", function!(remove_formatting, 1));
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_remove_formatting() {
    assert_eq!(remove_formatting("Καλημέρα"), "Kalemera");
    assert_eq!(remove_formatting("Ήθος"), "Ethos");
    assert_eq!(remove_formatting("Έθος"), "Ethos");
    assert_eq!(remove_formatting("Χριστίνα"), "Khristina");
  }
}