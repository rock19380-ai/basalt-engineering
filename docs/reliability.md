# Reliability concerns

Basalt treats the transaction itself as only one part of a payment-operations system. Production concerns include idempotent invoice creation, payment verification convergence, webhook retry/replay handling, worker recovery, approval separation, partial/failed batch outcomes, reconciliation, audit events, and finance-oriented exports.

Plan/subscription work and invoice work are isolated so shared changes do not silently regress the existing invoice payer path.
