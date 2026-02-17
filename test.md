**Title:** MFN vs Managed Fleets - Quick Comparison

| Aspect | Managed Fleets (MF) | Managed Fleets Next (MFN) |
|--------|---------------------|---------------------------|
| **Capacity Type** | Prod, Substrate, FHI, Hailstone | NAWS (EC2 ASGs) |
| **Orchestration** | Foundry + Garmin (SWF) | Shoemaker (State Machine) |
| **UI** | Foundry Website (regional) | Global Website |
| **IaC Integration** | N/A (uses Apollo/Provisioning) | Managed ASG (CFN/CDK) |
| **Patching Method** | Rebuild in-place | Instance Refresh |
| **Operator Access** | FoundryOps hosts | ShoemakerTools hosts |
| **Service Scope** | Regional | Partitional |
