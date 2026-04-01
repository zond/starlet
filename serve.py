#!/usr/bin/env python3
import http.server
import ssl
import os
import sys

PORT = int(sys.argv[1]) if len(sys.argv) > 1 else 8443
DIR = os.path.dirname(os.path.abspath(__file__))
CERT = os.path.join(DIR, "cert.pem")
KEY = os.path.join(DIR, "key.pem")

os.chdir(DIR)

server = http.server.HTTPServer(("0.0.0.0", PORT), http.server.SimpleHTTPRequestHandler)
ctx = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
ctx.load_cert_chain(CERT, KEY)
server.socket = ctx.wrap_socket(server.socket, server_side=True)

print(f"Serving on https://0.0.0.0:{PORT}")
server.serve_forever()
