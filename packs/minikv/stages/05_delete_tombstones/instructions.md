# Delete tombstones

Add:

```bash
minikv delete-log <path> <key>
```

Append a tombstone line to the log:

```txt
DEL key
```

`get` should treat the latest tombstone for a key as a successful empty result. `compact` should omit deleted keys.

Edge cases:

- deleting a missing key is still a valid tombstone
- repeated deletes should not create live values
- recovery should use the latest operation for a key

Non-goals:

- transactions
- concurrent writers
- binary log encoding
