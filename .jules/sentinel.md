## 2025-05-15 - Hardcoded configuration and secret leakage risk
**Vulnerability:** Found a hardcoded "placeholder" string `VESPER_CF_ID` in the Cloudflare API base URL within `server_api.rs`. Additionally, sensitive configuration file `.ENV` was tracked by Git, and a root `.gitignore` was missing, posing a high risk of secret leakage.
**Learning:** In forks or white-label transitions, placeholders meant for interpolation can easily be left as hardcoded literals if not properly refactored to use environment variables. Missing repository-level ignore rules for environment files is a critical security gap.
**Prevention:** Always use `std::env::var` for configuration that varies by environment (IDs, Keys). Ensure a root `.gitignore` and `.ENV.example` are present at the start of a project or fork.

## 2025-05-15 - CSS Injection via style tag
**Vulnerability:** The `ChartStyle` component in `landingpage/components/ui/chart.tsx` was using `dangerouslySetInnerHTML` to inject CSS containing unsanitized `id`, `key`, and `color` props.
**Learning:** Even internal-looking UI components can be vectors for CSS injection (which can lead to data exfiltration via `url()` or UI spoofing) if they dynamically build style tags from props without sanitization.
**Prevention:** Always sanitize any variable being injected into a `<style>` tag. Use strict regex to allow only safe characters (e.g., alphanumeric, dashes, underscores) for CSS identifiers and strip potentially harmful characters (e.g., `;`, `{`, `}`, `\`, `<`) from values.
