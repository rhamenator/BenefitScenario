use benefit_scenario::{Plan, annual_cost};

fn main() {
    let cost = annual_cost(
        Plan {
            monthly_premium_cents: 80_000,
            employee_share_basis_points: 3_000,
            deductible_cents: 150_000,
            coinsurance_basis_points: 2_000,
            out_of_pocket_max_cents: 450_000,
        },
        600_000,
    )
    .unwrap();
    println!(
        "employee annual scenario: ${:.2}",
        cost.employee_total_cents as f64 / 100.0
    );
}
