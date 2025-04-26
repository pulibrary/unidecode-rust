# unidecode_rust

This is an experiment at replacing the Ruby
stringex gem to a new gem that uses the unidecode rust
crate.

We hope that it will have a much lower memory footprint
in our application.

To run tests:
1. Install cargo+rust
2. `bundle install`
3. `bundle exec rake`

To compile:
1. Install cargo+rust
2. `bundle install`
3. `bundle exec rake compile`

To build and install locally:
1. gem build unidecode_rust.gemspec
2. gem install unidecode_rust-0.1.0.gem

To try it out:
1. `bundle exec rake console`
```
irb(main):001:0> Unidecoder.unidecode 'Καλημέρα'
=> "Kalemera"
```
