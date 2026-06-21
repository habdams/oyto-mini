use anchor_lang::prelude::*;
use oyto_mini::state::*;

#[test]
fn test_compensation_amount_from_rule() {
    let rule = RuleAccount {
        event_type: EventType::MergedPr,
        reward: 10,
    };

    let compensation = CompensationAccount {
        contributor: Pubkey::new_unique(),
        amount: rule.reward,
    };

    assert_eq!(compensation.amount, 10);
}

#[test]
fn test_different_rules_generate_different_rewards() {
    let merged_pr_rule = RuleAccount {
        event_type: EventType::MergedPr,
        reward: 10,
    };

    let review_rule = RuleAccount {
        event_type: EventType::Review,
        reward: 3,
    };

    assert_eq!(merged_pr_rule.reward, 10);

    assert_eq!(review_rule.reward, 3);

    assert_ne!(merged_pr_rule.reward, review_rule.reward);
}
