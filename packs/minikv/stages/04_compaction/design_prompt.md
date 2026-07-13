# Design Prompt — Compaction without changing meaning

Before implementing compaction, write a short note answering:

1. What does it mean for the input and output logs to be logically equivalent?
2. Which historical records can be discarded with certainty?
3. Why is deterministic key ordering useful even though replay semantics do not require it?
4. What could a reader observe if the output were replaced halfway through a write?
5. How would your design change if the input and output paths were allowed to be the same?
6. Which benchmark cost do you expect to dominate: replay, allocation, sorting, or writing?

Revisit the note after `deltaforge bench`. Record which expectation was wrong and what evidence changed your mind.
