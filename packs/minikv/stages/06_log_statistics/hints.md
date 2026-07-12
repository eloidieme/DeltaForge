# Hint 1

Separate physical facts about records from the logical fact of which keys are live at the end.

# Hint 2

Count each parsed operation while replaying it into the same latest-state model used by `get` and `compact`.

# Hint 3

Maintain `entries` and `tombstones` counters during the scan, then count `Some` values in the recovered map for `live_keys`.
