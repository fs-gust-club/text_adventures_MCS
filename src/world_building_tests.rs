use super::Room;

#[test]
fn test_add_exit() {
    // Arrange
    let mut victim = Room::new("id".to_string(), "description".to_string());
    let destination = Room::new("dest".to_string(), "another description".to_string());

    // Act
    victim.add_exit("direction".to_string(), &destination);
    let result = victim.get_exits().any(|s| s == "direction");
    // Assert
    assert_eq!(result, true);
}
