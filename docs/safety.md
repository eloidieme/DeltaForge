# Safety

DeltaForge does not run learner commands through a shell. Test and benchmark commands are executed as argument vectors.

Fixtures are copied to temporary directories before execution. State, config, reports, and benchmark history use atomic writes. DeltaForge avoids destructive filesystem operations outside known temp or project paths.
