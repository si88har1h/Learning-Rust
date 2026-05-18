fn main() {
    let aov: f64 = 1250.0;
    const CAC: u32 = 250;
    let cogs_pct: f64 = 0.35; //mutability
    let repeat_orders: u32 = 15;
    const GST: f64 = 0.18;

    let net_aov: f64 = aov / (1.0 + GST);
    // mutation and make up rise in cost due to import duty
    println!("cogs: {cogs_pct}");
    let cogs_pct: f64 = 0.35 + 0.10;
    let gross_profit_per_order: f64 = net_aov * (1.0 - cogs_pct);
    let payback_in_orders: f64 = CAC as f64 / gross_profit_per_order;
    let ltv: f64 = gross_profit_per_order * (repeat_orders as f64);

    let metrics: (f64, f64, f64, f64) = (net_aov, gross_profit_per_order, payback_in_orders, ltv);

    println!(
        "Metrics are as follows: Net aov : {0:.2}, Gross Profit : {1:.2}, Payback: {2:.2}, LTV: {3:.3}",
        metrics.0, metrics.1, metrics.2, metrics.3
    );
}
