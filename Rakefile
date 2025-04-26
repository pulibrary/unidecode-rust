require 'rake/extensiontask'
require 'rspec/core/rake_task'

Rake::ExtensionTask.new("unidecode_rust") do |c|
  c.lib_dir = "lib/unidecode_rust"
end

task :dev do
  ENV['RB_SYS_CARGO_PROFILE'] = 'dev'
end

task default: 'test'
task test: [:rust_test, :ruby_test]
task :rust_test do
  Dir.chdir("#{__dir__}/ext/unidecode_rust") do
    raise 'Rust tests failed' unless system 'cargo test'
  end
end

RSpec::Core::RakeTask.new(:ruby_test)
task ruby_test: :compile

task console: :compile do
  require "unidecode_rust/unidecode_rust"
  require "irb"
  ARGV.clear
  IRB.start
end
