use text_adventure::world_building::{Item, Room, World};

#[test]
fn test_add_exit() {
    // Arrange
    let mut victim = Room::new("id".to_string(), "description".to_string());

    // Act
    victim.add_exit("direction".to_string(), "room".to_string());
    let result = victim.get_exits().any(|s| s == "direction");
    // Assert
    assert_eq!(result, true);
}
