# Hint 1

A measurement has two boundaries: decide exactly which existing work begins after the timer starts and ends before it is read.

# Hint 2

Reuse corpus selection for the file count and keep serialization as the final, untimed-or-consistently-timed step.

# Hint 3

`Instant::now().elapsed().as_millis()` is monotonic; emit the two numeric fields with a JSON serializer or carefully fixed formatting and no extra stdout.
