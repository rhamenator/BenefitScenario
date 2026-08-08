#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Plan {
    pub monthly_premium_cents: u64,
    pub employee_share_basis_points: u32,
    pub deductible_cents: u64,
    pub coinsurance_basis_points: u32,
    pub out_of_pocket_max_cents: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnnualCost {
    pub employee_premium_cents: u64,
    pub employer_premium_cents: u64,
    pub employee_claim_cost_cents: u64,
    pub employee_total_cents: u64,
    pub employer_total_cents: u64,
}

pub fn annual_cost(
    plan: Plan,
    expected_allowed_claims_cents: u64,
) -> Result<AnnualCost, &'static str> {
    if plan.employee_share_basis_points > 10_000 || plan.coinsurance_basis_points > 10_000 {
        return Err("basis-point shares cannot exceed 10000");
    }
    let annual_premium = plan.monthly_premium_cents.saturating_mul(12);
    let employee_premium_cents = basis_points(annual_premium, plan.employee_share_basis_points);
    let employer_premium_cents = annual_premium - employee_premium_cents;
    let after_deductible = expected_allowed_claims_cents.saturating_sub(plan.deductible_cents);
    let employee_claim_cost_cents = (plan.deductible_cents.min(expected_allowed_claims_cents)
        + basis_points(after_deductible, plan.coinsurance_basis_points))
    .min(plan.out_of_pocket_max_cents);
    Ok(AnnualCost {
        employee_premium_cents,
        employer_premium_cents,
        employee_claim_cost_cents,
        employee_total_cents: employee_premium_cents + employee_claim_cost_cents,
        employer_total_cents: employer_premium_cents,
    })
}

fn basis_points(amount: u64, rate: u32) -> u64 {
    ((u128::from(amount) * u128::from(rate) + 5_000) / 10_000) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn annual_scenario_respects_deductible_coinsurance_and_oop_max() {
        let cost = annual_cost(
            Plan {
                monthly_premium_cents: 100_000,
                employee_share_basis_points: 2_500,
                deductible_cents: 200_000,
                coinsurance_basis_points: 2_000,
                out_of_pocket_max_cents: 500_000,
            },
            2_000_000,
        )
        .unwrap();
        assert_eq!(cost.employee_premium_cents, 300_000);
        assert_eq!(cost.employee_claim_cost_cents, 500_000);
        assert_eq!(cost.employee_total_cents, 800_000);
    }
}
