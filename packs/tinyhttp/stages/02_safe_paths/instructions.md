# Stage 04 — Keep paths inside the document root

## Goal

Prevent a request path from escaping the configured public directory while preserving normal static-file responses.

Static serving maps a client path to a file. Path validation makes the boundary around that mapping explicit.

## Background

Suppose the server is allowed to publish this directory:

```text
site/
├── public/
│   └── index.html
└── secret.txt
```

The operator chose `site/public` as the **document root**. `index.html` is public. `secret.txt` is not.

A request for `/../secret.txt` asks the path resolver to begin in `public`, move to its parent, and open `secret.txt`:

```text
public/../secret.txt → site/secret.txt
```

If TinyHTTP joins that client text to the root without inspecting its components, it can reveal a file the operator never meant to serve. This class of mistake is called directory traversal.

Checking only the beginning of the string is not enough. Parent components can appear after an ordinary directory, as in `/assets/../../secret.txt`. The security decision belongs to the path's components, not to a search for one convenient substring.

TinyHTTP rejects parent, platform-prefix, and root-escaping components before the file is read. This conservative rule is narrower than a complete URL-to-filesystem policy, which would also address URL decoding, symbolic links, canonicalization races, and operating-system-specific namespaces.

On failure, the outside file's contents must never appear on stdout. A security check that reports an error after reading or printing the secret is too late.

## Requirements

Keep `tinyhttp serve-file <root> <request-path>` and the existing `200`/`404` response behavior for safe paths.

Reject any request path whose components attempt to move to a parent, introduce an external filesystem root, or use a platform path prefix. Exit non-zero, include `unsafe path` in stderr, and never reveal bytes from outside `<root>`.

## Example

```console
$ tinyhttp serve-file site/public /../secret.txt
error: unsafe path
```

`site/secret.txt` is never returned.

## Edge cases

- A leading parent traversal is rejected.
- Parent traversal nested after an ordinary component is rejected.
- A normal root-relative request still serves a public file.
- Failure output does not contain the outside file's contents.

## Success criteria

All `deltaforge test` cases pass and every successfully served file remains beneath the chosen document root under the component policy.

## Non-goals

- URL percent-decoding.
- Following or defending against filesystem symlinks in the served tree.
- Authentication and per-file permissions.
- A complete cross-platform URL-to-path standard.
