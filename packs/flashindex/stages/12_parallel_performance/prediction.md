# Prediction

This measurement runs the same indexing workload at one, two, four, and eight threads,
and the target is a speedup of at least 1.5 between the slowest and fastest thread
counts.

Before measuring, predict the shape of the curve rather than a single number. Does the
time keep halving as threads double, flatten out after a certain count, or get worse
past some point? Name the count where you expect the curve to bend, and say what you
think causes the bend: the serial part of the work, contention on something shared, or
the cost of splitting and recombining.
