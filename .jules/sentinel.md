## 2025-05-16 - [CSS Injection in Chart Component]
**Vulnerability:** CSS injection via unsanitized properties (`id`, `key`, `color`) in a component using `dangerouslySetInnerHTML` for a `<style>` tag.
**Learning:** Even internal UI components can be vulnerable if they dynamically generate CSS from props. In this case, `recharts` chart components were vulnerable to escaping the style tag and executing scripts or altering UI.
**Prevention:** Always sanitize any dynamic data before injecting it into a `<style>` tag or using `dangerouslySetInnerHTML`. Use restrictive regex for identifiers and strip control characters for values.
