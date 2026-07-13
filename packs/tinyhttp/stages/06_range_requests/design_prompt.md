# Design Prompt — Partial representations

Before implementing ranges, answer:

1. Why is the end offset inclusive, and where can that cause an off-by-one error?
2. Which length describes the complete file and which length describes the response body?
3. In what order should path safety, number parsing, bounds checks, and file reads occur?
4. What response information lets a client place this fragment in the full representation?
5. How would suffix or multiple ranges complicate your current response model?
6. Which parts of the document-root security boundary must remain unchanged?
