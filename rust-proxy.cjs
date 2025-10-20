#!/usr/bin/env node
/**
 * Simple HTTP proxy to add Origin headers for Rust MCP server
 * Listens on port 5053 and proxies to Rust server on port 3001
 */

const http = require('http');

const RUST_SERVER = 'http://localhost:3001';
const PROXY_PORT = 5053;

const server = http.createServer((clientReq, clientRes) => {
    console.log(`${clientReq.method} ${clientReq.url} from ${clientReq.socket.remoteAddress}`);

    // Proxy options
    const options = {
        hostname: 'localhost',
        port: 3001,
        path: clientReq.url,
        method: clientReq.method,
        headers: {
            ...clientReq.headers,
            'origin': 'http://localhost',  // Add Origin header
            'host': 'localhost:3001'        // Update host header
        }
    };

    // Forward request to Rust server
    const proxyReq = http.request(options, (proxyRes) => {
        // Add CORS headers to response
        clientRes.writeHead(proxyRes.statusCode, {
            ...proxyRes.headers,
            'Access-Control-Allow-Origin': '*',
            'Access-Control-Allow-Methods': 'GET, POST, DELETE, OPTIONS',
            'Access-Control-Allow-Headers': 'Content-Type, Origin'
        });

        proxyRes.pipe(clientRes);
    });

    proxyReq.on('error', (err) => {
        console.error('Proxy error:', err.message);
        clientRes.writeHead(502);
        clientRes.end('Bad Gateway');
    });

    clientReq.pipe(proxyReq);
});

server.listen(PROXY_PORT, '0.0.0.0', () => {
    console.log(`\n╔══════════════════════════════════════╗`);
    console.log(`║    Rust MCP Server Proxy            ║`);
    console.log(`╚══════════════════════════════════════╝`);
    console.log(`Proxy:  http://0.0.0.0:${PROXY_PORT}`);
    console.log(`Target: ${RUST_SERVER}`);
    console.log(`Ready for connections\n`);
});
