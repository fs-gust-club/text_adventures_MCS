use text_adventure::world_building::{World, Room, Item};

#[test]
fn test_add_room() {
    assert_eq!(true, true);
}

#[test]
fn test_add_exit() {
    // Arrange
    let victim = Room::new("id".to_string(), "description".to_string());

    // Act
    victim.add_exit("direction", "room");
    let result = victim.get_exits().any("direction".to_string().as_ref());
    
    // Assert
    assert_eq!(result, true);
}