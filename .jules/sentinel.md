## 2025-05-16 - [CRITICAL] Exposed Secrets in Version Control
**Vulnerability:** Sensitive API keys (Cloudflare, Groq) were being tracked by Git in the root `.ENV` file.
**Learning:** Initial environment setup sometimes leads to accidental tracking of `.env` files if a root-level `.gitignore` is missing or incomplete.
**Prevention:** Always ensure a root-level `.gitignore` explicitly excludes `.ENV` and provide a `.ENV.example` template with redacted placeholders.

## 2025-05-16 - [HIGH] CSS Injection in Chart Component
**Vulnerability:** The `ChartStyle` component in `landingpage/components/ui/chart.tsx` used `dangerouslySetInnerHTML` to inject dynamic styles using `id`, `key`, and `color` props without sanitization.
**Learning:** Dynamic style generation from props is a common pattern that can introduce CSS injection if the values are not strictly validated or sanitized.
**Prevention:** Sanitize identifiers with regex (e.g., `/[^a-zA-Z0-9-_]/g`) and strictly validate color values against a safe whitelist or regex.
