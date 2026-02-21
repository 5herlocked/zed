#!/usr/bin/env python3
"""Dev server for zed_web that adds SharedArrayBuffer headers.

parking_lot atomics in WASM require SharedArrayBuffer, which browsers
only enable when the page is cross-origin isolated. This server adds
the required COOP and COEP headers.

Usage: python3 serve.py [port]
"""
import http.server
import sys

PORT = int(sys.argv[1]) if len(sys.argv) > 1 else 3000

class Handler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header("Cross-Origin-Opener-Policy", "same-origin")
        self.send_header("Cross-Origin-Embedder-Policy", "require-corp")
        self.send_header("Cross-Origin-Resource-Policy", "cross-origin")
        super().end_headers()

    def do_GET(self):
        # Serve WASM with correct MIME type
        if self.path.endswith(".wasm"):
            self.send_response(200)
            self.send_header("Content-Type", "application/wasm")
            self.end_headers()
            with open(self.translate_path(self.path), "rb") as f:
                self.wfile.write(f.read())
            return
        super().do_GET()

print(f"Serving on http://localhost:{PORT} (with COOP/COEP headers)")
http.server.HTTPServer(("", PORT), Handler).serve_forever()
