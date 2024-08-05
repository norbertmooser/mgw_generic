| Current State | Event              | Action               | Next State |
|---------------|--------------------|----------------------|------------|
| Idle          | Initialization     | None                 | Ping       |
| Ping          | Ping Successful    | None                 | Socket     |
| Ping          | Ping Failed        | Log failure          | Idle       |
| Socket        | Socket Exists      | None                 | Read       |
| Socket        | No Socket          | Attempt to establish | Read       |
| Socket        | Socket Setup Fail  | Log failure          | Idle       |
| Read          | Read Successful    | Process data         | Write      |
| Read          | Read Fail          | Log failure          | Idle       |
| Write         | Write Successful   | Confirm write        | Verify     |
| Write         | Write Fail         | Log failure          | Idle       |
| Verify        | Verification Pass  | Log success          | Idle       |
| Verify        | Verification Fail  | Log failure          | Idle       |
