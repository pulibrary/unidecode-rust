use magnus::{function, Error, Ruby, Object};

fn unidecode(a: String) -> String {
  unidecode::unidecode(&a)
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
  let module = ruby.define_module("Unidecoder")?;
  module.define_singleton_method("unidecode", function!(unidecode, 1))?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_unidecode() {
    assert_eq!(unidecode("ø".to_string()), "o");
    assert_eq!(unidecode("Καλημέρα".to_string()), "Kalemera");
    // 'Ηθος and Έθος are two different words with different meaning. 
    // They should not be decoded to the same latin characters word Ethos.
    assert_eq!(unidecode("Ήθος".to_string()), "Ethos");
    assert_eq!(unidecode("Έθος".to_string()), "Ethos");
    assert_eq!(unidecode("Χριστίνα".to_string()), "Khristina");
  }
}
