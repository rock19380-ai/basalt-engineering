# Basalt Engineering Showcase

**Solana-first, non-custodial stablecoin payment-operations infrastructure.**

This public repository documents selected engineering concepts from Basalt without mirroring its private production source.

## Product boundary

Basalt is an operations layer around stablecoin payments. It is designed to create payment records/invoices, verify on-chain transfers, deliver signed webhooks, enforce organization and role boundaries, coordinate approval-gated payouts, reconcile batch payout outcomes, and maintain audit-grade operational records.

Basalt is **non-custodial**: production architecture does not depend on the server owning customer private keys or taking custody of customer funds.

## Core workflow

```text
Platform creates invoice
        |
        v
Customer pays configured recipient wallet directly
        |
        v
Basalt verifies on-chain payment
        |
        v
Signed webhook / operational state
        |
        v
Platform continues business workflow
```

Payout operations keep proposal, approval, wallet signing, submission, reconciliation, and audit responsibilities distinct.

## Public example

`examples/approval-demo` is an independently written state-machine demonstration of role-separated payout approval. It contains no wallet signing and no production Basalt source.
