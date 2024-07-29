// integration test file
use adder_tester_part2;

#[test]
fn it_adds_two(){
    assert_eq!(adder_tester_part2::add_two(2),4)
}