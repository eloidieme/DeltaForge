# Learning-content style

DeltaForge teaches by making a problem understandable before naming the tool that solves it. Pack content should read like a patient technical book, not an API reference with friendly adjectives added afterward.

## Editorial baseline

The baseline voice comes from the FlashIndex overview and Stage 01 guide.

1. Begin with a concrete project, command, input, or failure the learner can picture.
2. Let the problem become clear before introducing its customary name or solution.
3. Define a term next to the first example that needs it. Do not lead with a glossary.
4. Explain why an observable rule exists. If a list, threshold, format, or policy is specific to the tool rather than a universal standard, describe the boundary and the tradeoff in the world of the project.
5. Prefer ordinary sentences and concrete nouns. Avoid slogans, ornamental cleverness, promotional language, and compressed encyclopedia prose.
6. Use examples to carry the explanation forward. A diagram should clarify the prose rather than stand in for it.
7. Keep implementation freedom. Explain the behavior and the reasoning behind it without giving a construction recipe.
8. Write as though the current text were the first edition. Learner-facing prose must not mention rewrites, inserted or split stages, prior wording, test-driven policy choices, or decisions made to answer editorial feedback.

## Shape of an overview

An overview starts with a representative situation in which the finished tool would help. It then introduces the questions the project must answer, in dependency order. Only after the questions are visible should it name larger concepts such as an inverted index, compaction, HTTP framing, or a call stack.

The overview should help a new learner answer:

- What will I be able to do when this is finished?
- Why can the finished behavior not be built in one undifferentiated step?
- What form does the information take as it moves through the project?
- Which decisions are project-specific simplifications?

Historical notes, glossaries, extensions, and diagnostic exercises remain useful secondary material. They should not interrupt the opening explanation. An overview may describe the path from one capability to another, but it should not narrate how the curriculum was assembled or revised.

## Shape of a stage

Every stage keeps the seven required sections in order:

1. `Goal`
2. `Background`
3. `Requirements`
4. `Example`
5. `Edge cases`
6. `Success criteria`
7. `Non-goals`

`Goal` names the one new observable ability. `Background` walks from a concrete case to the concept behind that ability. `Requirements` switches to exact contract language. `Example` demonstrates the complete contract without revealing an implementation. Every listed edge case must be exercised by a black-box test. `Non-goals` protects the learner from solving later stages early.

## Stage size

A stage should introduce one principal conceptual move. Several closely related output rules may travel together, but a stage should be split when it combines separate questions that deserve separate explanations or failure modes. Examples include writing an artifact versus reading it, producing a correct parallel result versus meeting a speed target, forming candidate search results versus ordering ties, and serving a file versus securing a client-controlled path.

The final reference implementation may support later behavior while proving an earlier stage. Earlier tests therefore focus on the promise introduced at that point; later stages tighten the cumulative contract.
