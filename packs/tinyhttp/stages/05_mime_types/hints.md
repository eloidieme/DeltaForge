# Hint 1

This is a small classification table. Every successful file belongs to one named extension case or to the generic fallback.

# Hint 2

Choose the media type only after you know the response is successful. A missing path should follow the existing 404 path unchanged.

# Hint 3

`Path::extension().and_then(OsStr::to_str)` and a `match` can return the required static string without changing the response body.
