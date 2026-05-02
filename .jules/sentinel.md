## 2025-05-15 - Hardcoded Secrets and Tracked Configuration
**Vulnerability:** A root-level `.ENV` file containing live `CLOUDFLARE_API_KEY` and `GROQ_API_KEY`s was tracked in version control. Additionally, `VESPER_CF_ID` (Cloudflare Account ID) was hardcoded as a literal string placeholder in the backend code.
**Learning:** Initial white-labeling efforts or quick bootstrapping often leads to secrets leakage through temporary environment files or placeholder strings that are accidentally committed.
**Prevention:** Always implement a root `.gitignore` early, use `.env.example` templates, and verify that all infrastructure identifiers (Account IDs, Project IDs) are sourced from environment variables rather than being hardcoded.
