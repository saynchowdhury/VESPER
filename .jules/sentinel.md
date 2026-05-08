## 2024-05-24 - Exposed .ENV file and hardcoded identifiers
**Vulnerability:** A `.ENV` file containing live Cloudflare and Groq API keys was tracked by Git. Additionally, a Cloudflare account ID was hardcoded in the backend source code.
**Learning:** Local development environment files and cloud account identifiers were accidentally committed, exposing the application to unauthorized API usage and potential data leaks.
**Prevention:** Always use a root-level `.gitignore` to exclude `.ENV` files and build artifacts. Use environment variables for all sensitive configuration and account identifiers. Provide a `.ENV.example` template for developers.
