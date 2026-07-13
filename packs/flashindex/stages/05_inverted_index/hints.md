# Hint 1

Read one occurrence as a relationship: this token appears in this file. The index groups many such relationships under the token side.

# Hint 2

Choose a map whose keys are token strings and whose values can collect file paths. Transform the occurrence stream rather than scanning the source with a second token rule.

# Hint 3

A `BTreeMap<String, ...>` is a convenient standard-library starting point because its keys already have a stable traversal order. Its values still need to represent all containing paths.
