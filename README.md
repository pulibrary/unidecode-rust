# unidecode_rust

This is an experiment at moving our fork of the Ruby
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

To try it out:
1. `bin/console`
```
irb(main):001:0> remove_formatting 'Καλημέρα'
=> "Kalemera"
```
