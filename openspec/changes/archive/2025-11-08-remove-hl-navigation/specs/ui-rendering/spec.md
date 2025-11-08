# ui-rendering Specification Delta

## REMOVED Requirements

### Requirement: Three-Month Navigation
Navigation keys (H/L) MUST page through three-month periods instead of single months.

#### Scenario: Forward Paging
Given the calendar displays months X, X+1, X+2,
When pressing L (next),
Then display months X+3, X+4, X+5.

#### Scenario: Backward Paging
Given the calendar displays months X, X+1, X+2,
When pressing H (previous),
Then display months X-3, X-2, X-1.

#### Scenario: Year Boundary Handling
Given the current view ends in December,
When paging forward,
Then correctly display January of the next year.