# Non-custodial boundary

Basalt's architecture separates software orchestration from custody.

The system may create invoices, verify transfers, maintain operational state, prepare unsigned transaction flows, track approvals, reconcile results, and export records. It does not need to hold customer funds, own customer private keys, or make payout-entitlement decisions.

This boundary is both a security property and a product boundary: browser/client wallet signing remains distinct from backend operational authority.
