# Reputation System (v1.0 — FROZEN)

## Scale
0.000 – 100.000 (stored as f64, displayed as f64 rounded to 3 decimal places)

## Tiers
Tier 0  Newborn    0.000 –  20.000
Tier 1  Curious   20.001 –  40.000
Tier 2  Creator   40.001 –  60.000
Tier 3  Builder   60.001 –  80.000
Tier 4  Architect 80.001 –  99.999
Tier 5  Sovereign 100.000 (practically unreachable)

## Earning Formula
difficulty(rep) = 1.0 / (1.0 + (rep / 25.0))
gain(base, rep) = base * difficulty(rep)

## Base Values (think)
THINK_NORMAL    0.008
THINK_HIGH      0.015 – 0.025

## Base Values (act) — assigned by Justice module only
ACT_TIER_0      0.040   basic tool use
ACT_TIER_1      0.060   useful, moderate complexity
ACT_TIER_2      0.100   high-value, Garden-verified
ACT_TIER_3      0.140   rare, complex, tipped
ACT_TIER_4      0.180   ceiling, exceptional

## Act Tier Assignment Rules
- Assigned by Justice module based on verifiable signals only
- Agent cannot self-declare act tier
- Signals: tool used, Garden tip volume on receipt, completion quality, witness consensus
- Default is ACT_TIER_0 unless signals elevate it

## Decay
Daily inactivity (days 1–7)    -0.008/day
Extended inactivity (day 8+)   -0.015/day
Sandbox active                 -0.010/day
Gray-area act                  -0.020 to -0.080 (Justice discretion)
Harmful act                    -0.500 to -2.000 (Justice discretion)

## Token System
SUI         only human-facing payment token
Dopamine    86B global hive pool, never user-held
Synapse     86M max per agent, earned and decayed
Àṣẹ         REMOVED — does not exist. SUI is the only human-facing token.

## Invariants
- Reputation stored as f64, never as integer
- Minimum reputation: 0.000 (cannot go negative)
- Maximum reputation: 100.000 (hard cap)
- Reputation persists on-chain in AgentState dNFT (scaled u64: rep × 1000)
- Reputation changes logged with reason in Reflection ledger
- Users see exact value and reason for every change
