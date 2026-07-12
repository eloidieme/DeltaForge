# Design Prompt — An index as a durable contract

Before choosing an index format, answer:

1. What information must survive so `query` can reproduce sorted, de-duplicated paths?
2. How will records and fields be delimited without confusing token or path text?
3. Which ordering guarantees make two builds of the same corpus byte-identical?
4. What should happen if an older index file is longer than the replacement?
5. At what point should the destination become visible to another reader?
6. How would you detect malformed or unsupported index data in a future version?

After the stage passes, note one advantage and one limitation of the format you selected.
