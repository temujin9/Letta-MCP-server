
## 🔧 Redirect Handling Fix

This branch includes a fix for HTTP redirect handling that resolves connection issues with Letta servers.

### Problem Fixed
- MCP server was failing with "Unexpected non-whitespace character after JSON" errors
- Root cause: Letta server returns 307 redirects for endpoints without trailing slashes
- Axios wasn't following redirects, resulting in empty responses being parsed as JSON

### Solution Implemented
- Added `maxRedirects: 5` to axios configuration
- Added `validateStatus: (status) => status < 400` for proper status handling
- Updated unit tests to reflect new axios configuration

### Changes Made
- `src/core/server.js`: Added redirect handling to axios instance
- `src/test/core/api-client.test.js`: Updated test expectations
- `src/test/core/server-initialization.test.js`: Updated test expectations

### Testing
All existing tests pass with the new configuration. The fix resolves redirect issues without breaking existing functionality.

