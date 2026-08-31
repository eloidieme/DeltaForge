# Hint 1

Use the benchmark table to locate the shape of the problem. If two threads help but eight do not, look for serial work or contention rather than adding more threads blindly.

# Hint 2

Shared mutation in the per-file hot path often forces workers to wait. Worker-local accumulation followed by one merge usually gives independent work more room to overlap.

# Hint 3

Measure merge and output costs separately during diagnosis, but remove diagnostic stdout before testing. Preserve the exact benchmark corpus and canonical bytes while tuning how partial ordered postings are combined.
