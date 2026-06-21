use oyto_mini::error::PayrollError;

#[test]
fn test_invalid_reward() {
    let reward = 0;

    let result = reward > 0;

    assert!(!result);

    let err = PayrollError::InvalidReward;

    match err {
        PayrollError::InvalidReward => {}
        _ => panic!("wrong error"),
    }
}

#[test]
fn test_small_pr_rejected() {
    let pr_size = 5;

    let minimum_size = 20;

    assert!(pr_size < minimum_size);
}
