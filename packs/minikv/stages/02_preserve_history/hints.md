# Hint 1

The observable invariant is simple: every byte that existed before a successful write remains before the new record.

# Hint 2

Do not read and reconstruct the old file merely to add one line. Open the destination in a mode that positions writes at the end.

# Hint 3

`OpenOptions::new().create(true).append(true)` expresses the layout directly. Write one complete record per open command and let the next recovery stage interpret chronology.
