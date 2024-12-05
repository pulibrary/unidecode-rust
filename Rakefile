require "rake/extensiontask"

Rake::ExtensionTask.new("unidecode_rust") do |c|
  c.lib_dir = "lib/unidecode_rust"
end

task :dev do
  ENV['RB_SYS_CARGO_PROFILE'] = 'dev'
end

task default: 'test'
task :test do
  Dir.chdir("#{__dir__}/ext/unidecode_rust") do
    raise 'Rust tests failed' unless system 'cargo test'
  end
end
