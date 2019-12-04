use crate::entities::{Feature, Item, Room};

pub fn use_item(item: &Item, feature: &mut Feature, room: &mut Room) -> Result<String, String> {
    room.remove_feature(feature.get_name());
   // room.add_exit("up".to_string(), "hello".to_string());
    Ok("created door".to_owned())
}
