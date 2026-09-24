# Payment and payout lifecycles

## Invoice/payment

```text
invoice created
-> payer sees payment details
-> payer wallet sends to configured recipient
-> chain transaction is observed
-> amount/asset/recipient/reference are verified
-> invoice converges to paid state
-> signed webhook is delivered/retried
-> receipt/audit state remains inspectable
```

## Payout

```text
proposal
-> policy validation
-> approval/rejection
-> client-side wallet signing
-> submission
-> reconciliation
-> audit/export
```

The public showcase intentionally omits production transaction construction, credentials, internal rollout material, and private migration history.
