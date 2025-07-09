use godot::meta::PropertyInfo;
use godot::prelude::*;

#[derive(Default, GodotConvert, Export, Var)]
#[godot(via = GString)]
pub enum MapTypeDiscriminator {
    #[default]
    Circle,
}

pub enum MapType {
    Circle { map_radius: i32 },
}

impl Default for MapType {
    fn default() -> Self {
        Self::Circle { map_radius: 1 }
    }
}

#[derive(GodotClass)]
#[class(init, tool, base=Resource)]
struct HexagonMapTypeRust {
    map_type: MapType,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for HexagonMapTypeRust {
    fn get_property(&self, property: StringName) -> Option<Variant> {
        match self.map_type {
            MapType::Circle { map_radius } => {
                if property == StringName::from("type") {
                    Some(MapTypeDiscriminator::Circle.to_variant())
                } else if property == StringName::from("map_radius") {
                    Some(Variant::from(map_radius))
                } else {
                    None
                }
            }
        }
    }

    fn set_property(&mut self, property: StringName, value: Variant) -> bool {
        match self.map_type {
            MapType::Circle { .. } => {
                if property == StringName::from("type") {
                    match MapTypeDiscriminator::try_from_godot(value.stringify()) {
                        Ok(map_type) => match map_type {
                            MapTypeDiscriminator::Circle => true,
                        },
                        Err(err) => {
                            godot_error!("{}", err);
                            false
                        }
                    }
                } else if property == StringName::from("map_radius") {
                    self.map_type = MapType::Circle {
                        map_radius: value.to::<i32>(),
                    };
                    true
                } else {
                    false
                }
            }
        }
    }

    fn get_property_list(&mut self) -> Vec<PropertyInfo> {
        let mut properties = Vec::new();
        properties.push(PropertyInfo::new_export::<MapTypeDiscriminator>("type"));
        match &self.map_type {
            MapType::Circle { .. } => {
                properties.push(PropertyInfo::new_export::<i32>("map_radius"));
            }
        }
        properties
    }
}
