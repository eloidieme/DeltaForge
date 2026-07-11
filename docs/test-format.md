# Test Format

Stage tests live in `tests.yaml`.

```yaml
tests:
  - name: finds exact token matches
    fixture: search_project
    stdin: ""
    env:
      DELTAFORGE_EXAMPLE: "1"
    command: ["search", "{fixture_path}", "main"]
    expect:
      exit_code: 0
      stdout_contains:
        - "src/main.rs"
      stdout_not_contains:
        - "main_index"
      regex_match:
        - "(?m)^src/"
      file_exists:
        - "{temp_dir}/index.fi"
      file_contains:
        - path: "{temp_dir}/index.fi"
          contains: "main"
```

Supported expectations include `exit_code`, `stdout_exact`, `stdout_contains`, `stdout_not_contains`, `stderr_contains`, `file_exists`, `file_not_exists`, `file_contains`, `regex_match`, `json_equals`, and `timeout_ms`.

Tests may provide `stdin` and per-test `env` entries. Commands are executed directly as argument vectors, not through a shell.

Only a successful, unfiltered run of every test in a stage records completion. Filtered runs are diagnostic and never unlock progression. Test files reject unknown fields, zero timeouts, assertion-free cases, and fixture or expectation paths that escape the stage fixture/temporary root.
