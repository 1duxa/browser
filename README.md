# [Project Name(no idea)]

> A research browser: no HTML, no CSS, no JavaScript. A custom rendering engine,
> a custom scripting layer, and a custom document format, built from scratch to
> explore what a browser could look like if it were designed today instead of
> evolved from 1995 onward.

This is not an attempt to replace Chrome. It doesn't parse HTML, doesn't
implement CSS, and doesn't run JavaScript - and it never will. Those systems
solve problems this project doesn't have (cross-vendor markup compatibility,
30 years of backward compatibility, documents authored by people who've never
met). Cutting them out is a deliberate design choice, not a limitation to be
fixed later.

What's left is the actually interesting part of a browser: a GPU renderer, a
layout engine, and a scripting bridge - built around one core idea:
**explicit reactivity**. No virtual DOM, no diffing. Components call `ctx.draw()`
when they know something changed. The author is trusted to know their own state.

```rune
Div { class: "hello world", |ctx| {
    ctx.add(Text { "hello", ..Default::default() })
}}
```

## Why

Most "let's rethink the browser" projects either try to stay HTML/CSS/JS
compatible (and inherit all the complexity that implies), or go fully native
and abandon the "script drives a live tree" model that makes the web's
authoring experience what it is. This project keeps the tree-plus-script
model, but swaps every layer underneath it:

| Web platform | This project |
|---|---|
| HTML | A structural tree, authored directly in Rune |
| CSS | Layout & style as plain fields/functions, no cascade |
| JavaScript | [Rune](https://rune-rs.github.io/) |

## Status

Early and actively evolving. Currently working through the rendering
foundation before moving on to layout and scripting.

- [x] wgpu-based renderer, instanced draw calls
- [x] Growable GPU storage buffers (`VecBuf<T>`) with correct bind-group
      invalidation on resize
- [x] Basic text rendering
- [ ] SDF-based shape pipeline (fill, border, rounded corners, shadow — one
      shader, not one pipeline per effect)
- [ ] Clipping (scissor-rect, tree-order painting)
- [ ] Image pipeline
- [ ] Layout / box model
- [ ] Rune scripting bridge (`ctx.add`, `on_click`, explicit `ctx.draw()`)
- [ ] Manifest format (site graph: entry point, resources, navigation edges)
- [ ] Networking (HTTPS fetch of manifest + `.rune` files)

## Architecture

```
┌─────────────────────────────────────────┐
│              Rune scripts                │  authoring layer, explicit reactivity
├─────────────────────────────────────────┤
│         Component tree / layout          │  box model, no cascade
├─────────────────────────────────────────┤
│      Render pipelines (SDF / text /      │  wgpu, growable storage buffers
│           image, per-frame paint)        │
├─────────────────────────────────────────┤
│   Manifest-driven fetch (HTTPS, custom   │  "index.html" replaced by a
│         site-graph format)               │     dependency/navigation manifest
└─────────────────────────────────────────┘
```

### Rendering

Shapes are drawn as GPU-instanced storage-buffer arrays rather than one draw
call per element. Buffers grow on demand (`VecBuf<T>`).

### Scripting

[Rune](https://rune-rs.github.io/) replaces JavaScript. There is no virtual
DOM and no diffing pass - components call `ctx.draw()` explicitly when their
state changes, and event handlers (e.g. `on_click`) trigger redraws directly
rather than through an inferred dependency graph.

### Document format

There is no `index.html`. A site is described by a manifest: a graph of
files, the resources each one uses (images, fonts, other Rune modules), and
the navigation edges between them (which page can lead to which). The
manifest is authored, not inferred — the browser uses it to fetch, prefetch,
and precompile accordingly.

```ron
(
    entry: "main.rune",
    files: {
        "main": (
            path: "main.rune",
            resources: ["logo", "header_font"],
            leads_to: ["about", "contact"],
        ),
    },
    resources: {
        "logo": Image("assets/logo.png"),
        "header_font": Font("assets/inter-bold.ttf"),
    },
)
```

### Networking

Pages are fetched over HTTPS from a manifest-aware server. This is not an
attempt to load the existing web - arbitrary HTML is explicitly out of
scope. (Revisiting HTML interop via `html5ever` is a theoretical possibility
if this project ever needs to interpret existing content, but it is
deliberately not a current goal.)

## Non-goals

- **Web compatibility.** This will never load arbitrary existing websites.
- **CSS compliance.** Styling is plain data, not a cascade-resolving engine.
- **A JavaScript engine.** Rune is the scripting layer, permanently.

## Milestone

The first real validation target is rebuilding a simple, real page (a basic
blog post or docs page) using this stack, to surface what's actually missing
- expected candidates: text layout/line-breaking, scrolling, gradients, and
blur - rather than guessing requirements up front.

## Building

```sh
cargo run -- --mode square-test
```

## License

This program is released under the [GNU Affero General Public License v3.0](LICENSE).
