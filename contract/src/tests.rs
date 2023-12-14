use crate::actions::calculate_mint_fees;

#[test]
fn test_mint_fees() {
    let fees_tuples = [
        ("n", 200),
        ("na", 150),
        ("nam", 100),
        ("name", 50),
        ("names", 5),
        ("verylongname", 5),
    ];

    for (name, fee) in fees_tuples {
        let fees = calculate_mint_fees(name, 1);
        assert_eq!(fees, fee);
    }
}
