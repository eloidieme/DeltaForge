# Hint 1

Keep traversal intact and think of corpus selection as one yes-or-no question applied to each discovered regular file.

# Hint 2

The policy is an allow-list. A file is admitted only when its final extension exactly matches one of the listed lowercase strings; readable contents do not override the name.

# Hint 3

`Path::extension` returns an optional operating-system string. Convert it carefully and compare it with the exact allowed lowercase values.
