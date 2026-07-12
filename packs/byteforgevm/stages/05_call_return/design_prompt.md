# Design Prompt — Control state for subroutines

Before implementing calls, answer:

1. What exact address must `CALL` remember, and why?
2. Which values belong to the operand stack and which belong to call control state?
3. What invariant should hold between nested `CALL` and `RET` operations?
4. What state, if any, should change when a call target is invalid?
5. How would arguments and local variables change the minimal return-address model?
6. What should `HALT` mean when encountered inside a routine?
