from http.server import HTTPServer, SimpleHTTPRequestHandler

server_address = ('', 8000)

Handler = SimpleHTTPRequestHandler
Handler.extensions_map = {
  '.manifest': 'text/cache-manifest',
  '.html': 'text/html',
  '.png': 'image/png',
  '.jpg': 'image/jpg',
  '.svg': 'image/svg+xml',
  '.css': 'text/css',
  '.js': 'application/x-javascript',
  '.wasm': 'application/wasm',
  '': 'application/octet-stream', # Default
}

httpd = HTTPServer(server_address, Handler)
httpd.serve_forever()
