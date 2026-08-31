# Design Prompt — Parallel Indexing

Before implementing this stage, write a short note answering:

1. What work can be done independently per file?
2. What data structure is shared?
3. What data structure is thread-local?
4. Where might contention happen?
5. What speedup do you expect from 1 to 8 threads?
6. What might prevent perfect scaling?

Save your answer, then revisit it after you run `deltaforge bench`. Did the
merge phase cost more or less than you predicted? Where did the real time go?
