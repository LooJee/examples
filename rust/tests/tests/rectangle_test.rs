use tests;

#[test]
fn integrated_check_area() {
    let rect = tests::Rectangle {
        width: 2,
        height: 2,
    };

    //断言通过
    assert_eq!(4, rect.get_area());
    //断言通过
    assert_ne!(5, rect.get_area());
    //断言失败
    assert_eq!(5, rect.get_area());
}
