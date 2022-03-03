use crate::is_even;

#[test]
fn check_even_success(){
    assert!(is_even(6));
}

#[test]
fn check_even_failure(){
    assert_eq!(is_even(7),false);
}
