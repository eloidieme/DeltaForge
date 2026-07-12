# Design Prompt — The lifetime of a tombstone

Before implementing deletion, answer:

1. What state must replay retain for a key whose latest operation is `DEL`?
2. Why would simply removing that key from the recovery map be ambiguous during compaction?
3. Under what ordering does a later `SET` legitimately restore a deleted key?
4. What invariant prevents compaction from resurrecting an older value?
5. In a replicated store, when would it become safe to discard a tombstone entirely?

Keep the note focused on semantics and failure cases, not code structure.
