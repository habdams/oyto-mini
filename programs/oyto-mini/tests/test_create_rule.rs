use anchor_lang::prelude::*;
use oyto_mini::state::*;

#[test]
fn test_event_type_equality() {
    assert_eq!(EventType::MergedPr, EventType::MergedPr);

    assert_ne!(EventType::MergedPr, EventType::Review);
}

#[test]
fn test_rule_account_creation_values() {
    let rule = RuleAccount {
        event_type: EventType::MergedPr,
        reward: 10,
    };

    assert_eq!(rule.reward, 10);

    assert_eq!(rule.event_type, EventType::MergedPr);
}
