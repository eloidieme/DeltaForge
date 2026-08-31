# Hint 1

Measure the operation whose workload you report. Start the interval immediately before the corpus scan and stop it immediately after.

# Hint 2

Construct the two values first, then serialize one object. Any explanatory stdout outside the object makes the command unsuitable for machine parsing.

# Hint 3

`std::time::Instant` is monotonic and `elapsed().as_millis()` is non-negative. The JSON shape is small enough to format directly as long as it contains only the required integer fields.
