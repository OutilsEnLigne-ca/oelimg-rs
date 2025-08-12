#!/usr/bin/env python3
"""
Simple HTTP server to serve the Rust documentation locally.
Run this script and open http://localhost:8080 to view the docs.
"""

import http.server
import socketserver
import os
import webbrowser
import threading
import time

# Change to the target/doc directory
doc_dir = os.path.join(os.path.dirname(__file__), 'target', 'doc')

if not os.path.exists(doc_dir):
    print("❌ Documentation not found. Please run 'cargo doc' first.")
    exit(1)

os.chdir(doc_dir)

PORT = 8080
Handler = http.server.SimpleHTTPRequestHandler

def open_browser():
    """Open browser after a short delay"""
    time.sleep(1)
    webbrowser.open(f'http://localhost:{PORT}/oelimg_rs/')

# Start browser opening in a separate thread
browser_thread = threading.Thread(target=open_browser)
browser_thread.daemon = True
browser_thread.start()

print(f"🚀 Starting documentation server on http://localhost:{PORT}")
print(f"📖 Opening oelimg-rs documentation at http://localhost:{PORT}/oelimg_rs/")
print("Press Ctrl+C to stop the server")

with socketserver.TCPServer(("", PORT), Handler) as httpd:
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        print("\n👋 Shutting down server...")
        httpd.shutdown()