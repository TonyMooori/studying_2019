require "http/server"

server = HTTP::Server.new do |context|
  context.response.content_type  ="text/plain"
  context.response.print "hello world! time is #{Time.now}"
end

address = server.bind_tcp 8081
puts "Listening on http://#{address}"
server.listen

