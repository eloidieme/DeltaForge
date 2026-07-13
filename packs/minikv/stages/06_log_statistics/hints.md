# Hint 1

Count physical history while parsing, but compute live keys from the final replay state. They are intentionally different views.

# Hint 2

Every valid non-empty operation increments `entries`; every `DEL` also increments `tombstones`; only final `Some` values contribute to `live_keys`.

# Hint 3

Return both the recovered ordered map and the physical counters from one validated pass, or share one record parser between passes. Print only after parsing succeeds completely.
