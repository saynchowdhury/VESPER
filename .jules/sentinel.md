## 2025-05-15 - [CSS Injection in Chart Components]
**Vulnerability:** Unsanitized user-controlled properties (`id`, `key`, `color`) were injected directly into a `<style>` tag via `dangerouslySetInnerHTML` in the `ChartStyle` component.
**Learning:** Shared UI components that dynamically generate CSS from props are prime targets for injection if they use `dangerouslySetInnerHTML` or similar sinks. Even "safe" props like `color` can be used to break out of selectors if they contain characters like `}` or `;`.
**Prevention:** Always sanitize any variable before injecting it into a style or script block. Use strict alphanumeric filters for identifiers and strip control characters from values like colors.
