use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActorKind {
    ApiKey,
    HumanProposer,
    HumanApprover,
    HumanExecutor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PayoutState {
    Proposed,
    Approved,
    Rejected,
    AwaitingWalletSignature,
    Submitted,
    Settled,
    ReconciliationRequired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Payout {
    pub id: String,
    pub amount_atomic: u128,
    pub asset: String,
    pub recipient: String,
    pub state: PayoutState,
    approved_by: Option<String>,
}

impl Payout {
    pub fn propose(
        id: impl Into<String>,
        amount_atomic: u128,
        asset: impl Into<String>,
        recipient: impl Into<String>,
        actor: ActorKind,
    ) -> Result<Self, &'static str> {
        if actor != ActorKind::HumanProposer {
            return Err("human proposer required");
        }
        let asset = asset.into();
        let recipient = recipient.into();
        if amount_atomic == 0 || asset.trim().is_empty() || recipient.trim().is_empty() {
            return Err("invalid payout");
        }
        Ok(Self {
            id: id.into(),
            amount_atomic,
            asset,
            recipient,
            state: PayoutState::Proposed,
            approved_by: None,
        })
    }

    pub fn approve(
        &mut self,
        actor: ActorKind,
        approver_id: impl Into<String>,
    ) -> Result<(), &'static str> {
        if self.state != PayoutState::Proposed {
            return Err("payout is not proposed");
        }
        if actor != ActorKind::HumanApprover {
            return Err("human approver required");
        }
        self.approved_by = Some(approver_id.into());
        self.state = PayoutState::Approved;
        Ok(())
    }

    pub fn request_wallet_signature(&mut self, actor: ActorKind) -> Result<(), &'static str> {
        if actor != ActorKind::HumanExecutor {
            return Err("human executor required");
        }
        if self.state != PayoutState::Approved {
            return Err("approval required");
        }
        self.state = PayoutState::AwaitingWalletSignature;
        Ok(())
    }

    pub fn mark_submitted(&mut self) -> Result<(), &'static str> {
        if self.state != PayoutState::AwaitingWalletSignature {
            return Err("wallet signature required");
        }
        self.state = PayoutState::Submitted;
        Ok(())
    }

    pub fn reconcile(&mut self, chain_confirmed: Option<bool>) -> Result<(), &'static str> {
        if self.state != PayoutState::Submitted {
            return Err("submission required");
        }
        self.state = match chain_confirmed {
            Some(true) => PayoutState::Settled,
            Some(false) | None => PayoutState::ReconciliationRequired,
        };
        Ok(())
    }
}

#[derive(Default)]
pub struct IdempotencyGuard {
    keys: BTreeSet<String>,
}

impl IdempotencyGuard {
    pub fn claim(&mut self, key: impl Into<String>) -> Result<(), &'static str> {
        if self.keys.insert(key.into()) {
            Ok(())
        } else {
            Err("duplicate request")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn payout() -> Payout {
        Payout::propose(
            "payout-1",
            1_000,
            "USDC",
            "recipient-wallet",
            ActorKind::HumanProposer,
        )
        .unwrap()
    }

    #[test]
    fn api_key_cannot_approve() {
        let mut payout = payout();
        assert_eq!(
            payout.approve(ActorKind::ApiKey, "api"),
            Err("human approver required")
        );
    }

    #[test]
    fn wallet_signing_requires_prior_approval() {
        let mut payout = payout();
        assert_eq!(
            payout.request_wallet_signature(ActorKind::HumanExecutor),
            Err("approval required")
        );
    }

    #[test]
    fn happy_path_preserves_role_separation() {
        let mut payout = payout();
        payout
            .approve(ActorKind::HumanApprover, "approver-1")
            .unwrap();
        payout
            .request_wallet_signature(ActorKind::HumanExecutor)
            .unwrap();
        payout.mark_submitted().unwrap();
        payout.reconcile(Some(true)).unwrap();
        assert_eq!(payout.state, PayoutState::Settled);
    }

    #[test]
    fn unknown_submission_outcome_requires_reconciliation() {
        let mut payout = payout();
        payout
            .approve(ActorKind::HumanApprover, "approver-1")
            .unwrap();
        payout
            .request_wallet_signature(ActorKind::HumanExecutor)
            .unwrap();
        payout.mark_submitted().unwrap();
        payout.reconcile(None).unwrap();
        assert_eq!(payout.state, PayoutState::ReconciliationRequired);
    }

    #[test]
    fn duplicate_idempotency_key_is_rejected() {
        let mut guard = IdempotencyGuard::default();
        guard.claim("create:payout:1").unwrap();
        assert_eq!(guard.claim("create:payout:1"), Err("duplicate request"));
    }
}
