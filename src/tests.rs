use crate::main;

#[test]
fn add2test() 
{
    let b = 10;
    assert_eq!(b.clone() + 2, add2(b));
}