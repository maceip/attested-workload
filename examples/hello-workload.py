#!/usr/bin/env python3
"""Minimal co-located HTTP workload for Nitro app-proxy demos.

Listens on 127.0.0.1:8080 (MATCHER_LOOPBACK_PORT in src/net/vsock.rs).
The enclave TLS server forwards /v1/* and /healthz to this process.
"""

from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer


class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path == "/healthz":
            body = b'{"ok":true,"workload":"hello-workload"}'
        else:
            body = b'{"error":"use POST /v1/echo"}'
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def do_POST(self):
        length = int(self.headers.get("Content-Length", "0"))
        payload = self.rfile.read(length) if length else b""
        body = b'{"echo":' + payload + b"}"
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def log_message(self, *_args):
        pass


if __name__ == "__main__":
    host, port = "127.0.0.1", 8080
    server = ThreadingHTTPServer((host, port), Handler)
    print(f"hello-workload on http://{host}:{port}", flush=True)
    server.serve_forever()
