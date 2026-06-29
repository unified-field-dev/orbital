# orbital-discussion

Composable Leptos **Discussion** library — threaded reply trees (Tree, Flat, Compact view modes), markdown bodies, citations, attachments, composer, and agent reply parts.

Consumers own data via Leptos signals and wire backends through integration APIs (adapter trait, hooks, callbacks).

## Key types

- `DiscussionThread`, `DiscussionReply`, `DiscussionComposer`
- `DiscussionFeatures`, `DiscussionEvents`, `DiscussionAdapter`
- Markdown: [`orbital-markdown`](../orbital-markdown/)

## Preview

Local: `http://127.0.0.1:3010/orbital/discussion` (with `cargo leptos watch -p orbital-preview`).

## Scope

**In scope:** reply tree + composer, focus navigation, collapse/expand, citations, attachments, custom `*_view` callbacks, composition regions, agent part rendering.

**Out of scope:** conversation lists, feeds, moderation UI, HTTP/WebSocket transport (host wires those).

## Deferred (not in current charter)

- Auto-scroll, read receipts, unread highlight styling, image lightbox

## Docs

Consumer API: component rustdoc and preview catalog. CSS prefix: `orbital-discussion__*`. Use `default-features = false` in production; enable `preview` for the doc host.
