# Hint 1

Trace two calls on paper. The outer return address must remain below the inner one until the inner routine returns.

# Hint 2

Validate a `CALL` target before pushing anything. A rejected call should not leave a return address behind.

# Hint 3

Make `RET` use a fallible call-stack pop with `call stack underflow`. Keep `HALT` as an immediate loop exit regardless of call depth.
