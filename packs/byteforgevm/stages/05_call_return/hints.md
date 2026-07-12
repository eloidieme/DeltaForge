# Hint 1

Calls create a second last-in, first-out discipline: program values and control return points have different meanings and should not collide.

# Hint 2

On `CALL`, determine both destinations—the callee target and the caller's resume address—before mutating interpreter state.

# Hint 3

Use a separate `Vec<usize>`; push `ip + 1` after validating the target, and make `RET` pop or return `call stack underflow`.
