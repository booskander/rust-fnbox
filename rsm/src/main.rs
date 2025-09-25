mod balances;

fn main() {
    println!("Hello, world!");
}

#[test]
fn init_balances() {
    let mut pallet = balances::Pallet::new();

    assert_eq!(pallet.get_balance(&"Alice".to_string()), &0);
}

#[test]
fn fail_test() {
    assert_eq!(1, 2);
}
