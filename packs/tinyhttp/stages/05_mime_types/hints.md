# Hint 1

Media type selection is classification: every successful file must land in exactly one known case or the fallback.

# Hint 2

Determine the type from the validated file path, then pass it into the same response formatter used by Stage 02.

# Hint 3

`Path::extension().and_then(OsStr::to_str)` plus a small `match` gives a deterministic `&'static str` result.
