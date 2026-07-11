# Safety

DeltaForge does not run learner commands through a shell. Test and benchmark commands are executed as argument vectors.

Fixtures are copied to temporary directories before execution. State, config, reports, and benchmark history use atomic writes. DeltaForge avoids destructive filesystem operations outside known temp or project paths.

Learner output is drained concurrently and capped in captured diagnostics. Timeouts terminate the learner process group where the platform supports it. Pack, stage, template, fixture, and file-expectation paths are restricted to safe relative paths.

Projects pin the selected pack version, source directory, and content digest. A completed stage also records the pack and learner-tree digests that passed; changing either requires a fresh complete test run before `next` or `commit`.

Existing schema-version-1 projects created before integrity proofs remain readable. Their historical completed-stage entries are not accepted for progression until the learner reruns the complete stage tests, which records the missing proof without requiring a state-file migration.
