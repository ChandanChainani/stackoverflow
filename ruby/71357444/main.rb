require 'erb'

def render(file, &block)
  content = File.read(file)
  b = binding
  # ERB.new(content).run b
  ERB.new(content, trim_mode: '-').result b
end

res = render('file2.erb')
# puts "#{res.methods}"
puts res
