# Design Prompt — An index as a durable contract

Before writing the index, answer:

1. What information must survive so `query` can reproduce sorted, de-duplicated paths?
2. Why are tab-separated fields and newline-separated records unambiguous for the current token and path rules?
3. Which ordering guarantees make two builds of the same corpus byte-identical?
4. What should happen if an older index file is longer than the replacement?
5. At what point should the destination become visible to another reader?
6. Which change to token or path rules would require escaping or a new format version?

After the command works, note one advantage and one limitation of this line-oriented format.
