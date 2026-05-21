
## 2025-05-15 - [CRITICAL] Secret Exposure and Misconfigured API Endpoint
**Vulnerability:** The sensitive `.ENV` file containing Cloudflare and Groq API keys was tracked by Git and lacked a root-level `.gitignore`. Additionally, the Cloudflare API base URL contained a hardcoded literal placeholder instead of an environment variable.
**Learning:** Initial project setup for the Vesper fork neglected standard security hygiene for environment variables. Misalignment between environment variable names in `.ENV` and the Rust backend caused silent failures or hardcoded fallback behavior.
**Prevention:** Always implement a root `.gitignore` excluding `.ENV` and provide a `.ENV.example` template. Ensure all API endpoints use dynamic interpolation for account identifiers and keys.
