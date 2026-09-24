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

pub fn approve(state: PayoutState, human_approver: bool) -> Result<PayoutState, &'static str> {
    if state != PayoutState::Proposed { return Err("invalid state"); }
    if !human_approver { return Err("human approval required"); }
    Ok(PayoutState::Approved)
}

pub fn request_wallet_signature(state: PayoutState) -> Result<PayoutState, &'static str> {
    if state != PayoutState::Approved { return Err("approval required"); }
    Ok(PayoutState::AwaitingWalletSignature)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_style_actor_cannot_replace_human_approval() {
        assert_eq!(approve(PayoutState::Proposed, false), Err("human approval required"));
    }

    #[test]
    fn signing_is_after_approval() {
        assert_eq!(request_wallet_signature(PayoutState::Proposed), Err("approval required"));
        let approved = approve(PayoutState::Proposed, true).unwrap();
        assert_eq!(request_wallet_signature(approved).unwrap(), PayoutState::AwaitingWalletSignature);
    }
}
