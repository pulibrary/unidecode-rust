use magnus::{function, Error, Ruby};
use unidecode::unidecode;

fn remove_formatting(a: String) -> String {
  unidecode(&a)
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
  ruby.define_global_function("remove_formatting", function!(remove_formatting, 1));
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_remove_formatting() {
    assert_eq!(remove_formatting("ø".to_string()), "o");
    assert_eq!(remove_formatting("Καλημέρα".to_string()), "Kalemera");
    // 'Ηθος and Έθος are two different words with different meaning. 
    // They should not be decoded to the same latin characters word Ethos.
    assert_eq!(remove_formatting("Ήθος".to_string()), "Ethos");
    assert_eq!(remove_formatting("Έθος".to_string()), "Ethos");
    assert_eq!(remove_formatting("Χριστίνα".to_string()), "Khristina");
  }
}
