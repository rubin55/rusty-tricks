use std::fmt::Debug; // Import the Debug trait

// Define the enum with Debug trait bounds
pub enum SessionState<M: Debug, P: Debug> {
    MissingAttributes(M),
    ValidProposal(P),
}

fn log_session_event<M: Debug, P: Debug>(state: &SessionState<M, P>) {
    let (docs, r#type) = match state {
        SessionState::MissingAttributes(missing) => ("missing-docs", "missing-type"),
        SessionState::ValidProposal(proposal) => ("proposal-docs", "proposal-type"),
    };
    println!("I got docs: {}, type: {}", docs, r#type);
}

fn main() {
    // Using the enum with specific types that implement Debug
    let missing_state: SessionState<String, String> =
        SessionState::MissingAttributes("Missing 1".to_string());
    let proposal_state: SessionState<String, String> =
        SessionState::ValidProposal("Proposal 1".to_string());

    log_session_event(&missing_state);
    log_session_event(&proposal_state);
}
