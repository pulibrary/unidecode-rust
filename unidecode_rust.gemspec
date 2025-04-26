Gem::Specification.new do |spec|
  spec.name = 'unidecode_rust'
  spec.version = '0.1.0'
  spec.summary = 'Wrapper around the rust unidecode crate with extra PUL logic'
  spec.authors = ['Christina Chortaria', 'Jane Sandberg']
  spec.extensions = ["ext/unidecode_rust/extconf.rb"]
  spec.files = Dir["lib/**/*.rb", "ext/unidecode_rust/**/*.rs"] + ["ext/unidecode_rust/Cargo.toml"]

  # needed until rubygems supports Rust support is out of beta
  spec.add_dependency "rb_sys", "~> 0.9.39"

  # only needed when developing or packaging your gem
  spec.add_development_dependency "rake-compiler", "~> 1.2.0"
  spec.add_development_dependency "rspec"
end
