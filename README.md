# BenefitScenario

An organization-neutral employee benefit cost-sharing scenario engine derived
from HPlan's plan/group/employee/scenario ideas. It uses integer cents and basis
points and models annual premiums, deductible, coinsurance, and out-of-pocket
maximums. No employer or employee data is copied.

```powershell
cargo test
cargo run
```

Next slices: family tiers, employer contribution strategies, multiple claim
scenarios, group summaries, payroll-period projections, CSV import/export, and
clear assumptions. This is a planning tool, not actuarial or benefits advice.
