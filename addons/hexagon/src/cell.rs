use godot::global::PropertyUsageFlags;
use godot::meta::PropertyInfo;
use godot::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::sync::LazyLock;



#[derive(GodotClass)]
#[class(init)]
pub struct Direction;
#[godot_api]
impl Direction {
    #[constant]
    pub const TOP: i8 = 0;
    #[constant]
    pub const TOP_RIGHT: i8 = 1;
    #[constant]
    pub const RIGHT: i8 = 2;
    #[constant]
    pub const BOTTOM_RIGHT: i8 = 3;
    #[constant]
    pub const BOTTOM: i8 = 4;
    #[constant]
    pub const BOTTOM_LEFT: i8 = 5;
    #[constant]
    pub const LEFT: i8 = 6;
    #[constant]
    pub const TOP_LEFT: i8 = 7;
}

#[derive(GodotConvert, Var, Export, Debug, Eq, PartialEq, Hash)]
#[godot(via = i8)]
pub enum DirectionPointy {    
    TopRight = (Direction::TOP_RIGHT as isize),
    Right = (Direction::RIGHT as isize),
    BottomRight = (Direction::BOTTOM_RIGHT as isize),
    BottomLeft = (Direction::BOTTOM_LEFT as isize),
    Left = (Direction::LEFT as isize),
    TopLeft = (Direction::TOP_LEFT as isize),
}

#[derive(GodotConvert, Export, Var, Debug, Eq, PartialEq, Hash)]
#[godot(via = i8)]
pub enum DirectionFlat {
    Top = (Direction::TOP as isize),
    TopRight = (Direction::TOP_RIGHT as isize),
    BottomRight = (Direction::BOTTOM_RIGHT as isize),
    Bottom = (Direction::BOTTOM as isize),
    BottomLeft = (Direction::BOTTOM_LEFT as isize),
    TopLeft = (Direction::TOP_LEFT as isize),
}

fn calculate_axis(axis_1: i32, axis_2: i32) -> i32 {
    -axis_1 - axis_2
}

fn cube_round(q: f32, r: f32, s: f32) -> Cell {
    let mut rq = q.round();
    let mut rr = r.round();
    let rs = s.round();

    let x_diff = (rq - q).abs();
    let y_diff = (rr - r).abs();
    let z_diff = (rs - s).abs();

    if (x_diff > y_diff) & (x_diff > z_diff) {
        rq = -rr - rs
    } else if y_diff > z_diff {
        rr = -rq - rs
    }
    Cell::new_axial(rq as i32, rr as i32)
}

/// Hexagonal map cube position as describe here: https://www.redblobgames.com/grids/hexagons/#coordinates-cube
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, GodotClass)]
#[class(base=RefCounted, init)]
pub struct Cell {
    q: i32,
    r: i32,
}

static CUBE_DIRECTIONS_POINTY: LazyLock<HashMap<DirectionPointy, Cell>> = LazyLock::new(|| {
    HashMap::from([
        (DirectionPointy::TopRight, Cell::new_axial(1, -1)),
        (DirectionPointy::Right, Cell::new_axial(1, 0)),
        (DirectionPointy::BottomRight, Cell::new_axial(0, 1)),
        (DirectionPointy::BottomLeft, Cell::new_axial(-1, 1)),
        (DirectionPointy::Left, Cell::new_axial(-1, 0)),
        (DirectionPointy::TopLeft, Cell::new_axial(0, -1)),
    ])
});

static CUBE_DIRECTIONS_FLAT: LazyLock<HashMap<DirectionFlat, Cell>> = LazyLock::new(|| {
    HashMap::from([
        (DirectionFlat::Top, Cell::new_axial(0, -1)),
        (DirectionFlat::TopRight, Cell::new_axial(1, -1)),
        (DirectionFlat::BottomRight, Cell::new_axial(1, 0)),
        (DirectionFlat::Bottom, Cell::new_axial(0, 1)),
        (DirectionFlat::BottomLeft, Cell::new_axial(-1, 1)),
        (DirectionFlat::TopLeft, Cell::new_axial(-1, 0)),
    ])
});

#[godot_api]
impl Cell {
    pub fn zero() -> Self {
        Cell { q: 0, r: 0 }
    }

    #[func(rename=zero)]
    pub fn zero_gd() -> Gd<Self> {
        Gd::from_object(Self::zero())
    }

    /// Creates a position from axial coordinates
    pub fn new_axial(q: i32, r: i32) -> Self {
        // https://www.redblobgames.com/grids/hexagons/#conversions-axial
        Cell { q, r }
    }

    #[func(rename=new_axial)]
    /// Creates a position from axial coordinates
    pub fn new_axial_gd(q: i32, r: i32) -> Gd<Self> {
        // https://www.redblobgames.com/grids/hexagons/#conversions-axial
        Gd::from_object(Self::new_axial(q, r))
    }

    pub fn at_2d_position(pos: Vector2, cell_size: f32) -> Self {
        let q = (3_f32.sqrt() / 3_f32 * pos.x - 1_f32 / 3_f32 * pos.y) / (cell_size);
        let r = (2_f32 / 3_f32 * pos.y) / (cell_size);
        let s = -q - r;

        cube_round(q, r, s)
    }

    #[func(rename=at_2d_position)]
    pub fn at_2d_position_gd(pos: Vector2, cell_size: f32) -> Gd<Self> {
        Gd::from_object(Self::at_2d_position(pos, cell_size))
    }

    #[func]
    pub fn get_2d_position_for_flat_hexagon(&self, cell_size: f32) -> Vector2 {
        let x = cell_size * (3.0 / 2.0 * (self.get_q() as f32));

        // var y = size * (sqrt(3)/2 * hex.q  +  sqrt(3) * hex.r)
        let y = cell_size
            * (3.0_f32.sqrt() / 2.0 * (self.get_q() as f32)
                + 3.0_f32.sqrt() * (self.get_r() as f32));
        Vector2::new(x, y)
    }

    #[func]
    pub fn get_2d_position_point_for_pointy_hexagon(&self, cell_size: f32) -> Vector2 {
        let x = cell_size
            * (3.0_f32.sqrt() * (self.get_q() as f32)
                + 3.0_f32.sqrt() / 2.0 * (self.get_r() as f32));
        let y = cell_size * (3.0 / 2.0 * (self.get_r() as f32));
        Vector2::new(x, y)
    }

    #[func]
    pub fn get_even_q_offset_coordinates(&self) -> Vector2i {
        let col = self.q;
        let row = self.r + (self.q + (self.q & 1)) / 2;
        Vector2i::new(col, row)
    }

    pub fn from_even_q_offset_coordinates(offset_coords: Vector2i) -> Self {
        let q = offset_coords.x;
        let r = offset_coords.y - (offset_coords.x + (offset_coords.x & 1)) / 2;
        Self::new_axial(q, r)
    }

    #[func(rename=from_even_q_offset_coordinates)]
    pub fn from_even_q_offset_coordinates_gd(offset_coords: Vector2i) -> Gd<Self> {
        Gd::from_object(Self::from_even_q_offset_coordinates(offset_coords))
    }

    #[func]
    pub fn get_even_r_offset_coordinates(&self) -> Vector2i {
        let col = self.q + (self.r + (self.r & 1)) / 2;
        let row = self.r;
        Vector2i::new(col, row)
    }

    pub fn from_even_r_offset_coordinates(offset_coords: Vector2i) -> Self {
        let q = offset_coords.x - (offset_coords.y + (offset_coords.x & 1)) / 2;
        let r = offset_coords.y;
        Self::new_axial(q, r)
    }

    #[func(rename=from_even_r_offset_coordinates)]
    pub fn from_even_r_offset_coordinates_gd(offset_coords: Vector2i) -> Gd<Self> {
        Gd::from_object(Self::from_even_r_offset_coordinates(offset_coords))
    }

    // Create from Vector2 representation for easy passing to and from Godot
    pub fn from_vector2(vector: Vector2) -> Self {
        let q = vector.x as i32;
        let r = vector.y as i32;
        Cell::new_axial(q, r)
    }

    #[func(rename=from_vector2)]
    pub fn from_vector2_gd(vector: Vector2) -> Gd<Self> {
        Gd::from_object(Self::from_vector2(vector))
    }

    #[func]
    // Represent as Vector2 for easy passing to and from Godot
    pub fn as_vector2(&self) -> Vector2 {
        let x = self.q as f32;
        let y = self.r as f32;
        Vector2::new(x, y)
    }

    pub fn move_q(&self, length: i32) -> Cell {
        let new_q = self.q + length;
        Self::new_axial(new_q, self.get_r())
    }

    #[func(rename=move_q)]
    pub fn move_q_gd(&self, length: i32) -> Gd<Self> {
        Gd::from_object(self.move_q(length))
    }

    pub fn move_r(&self, length: i32) -> Self {
        let new_r = self.r + length;
        Self::new_axial(self.q - length, new_r)
    }

    #[func(rename=move_r)]
    pub fn move_r_gd(&self, length: i32) -> Gd<Self> {
        Gd::from_object(self.move_r(length))
    }
    pub fn move_s(&self, length: i32) -> Self {
        Self::new_axial(self.q - length, self.r)
    }

    #[func(rename=move_s)]
    pub fn move_s_gd(&self, length: i32) -> Gd<Self> {
        Gd::from_object(self.move_s(length))
    }

    pub fn get_q(&self) -> i32 {
        self.q
    }

    pub fn get_r(&self) -> i32 {
        self.r
    }

    pub fn get_s(&self) -> i32 {
        calculate_axis(self.q, self.r)
    }

    pub fn distance_to(&self, other: &Self) -> i32 {
        // https://www.redblobgames.com/grids/hexagons/#distances-axial
        ((self.q - other.q).abs() + (self.r - other.r).abs() + (self.get_s() - other.get_s()).abs())
            / 2
    }

    #[func(rename=distance_to)]
    pub fn distance_to_gd(&self, other: Gd<Self>) -> i32 {
        self.distance_to(&other.bind())
    }

    pub fn is_neighbour(&self, other: &Self) -> bool {
        self.distance_to(other) == 1
    }

    #[func(rename=is_neighbour)]
    pub fn is_neighbour_gd(&self, other: Gd<Self>) -> bool {
        self.is_neighbour(other.bind().deref())
    }

    pub fn get_neighbour_for_pointy_hex(&self, direction: DirectionPointy) -> Cell {
        let direction = CUBE_DIRECTIONS_POINTY[&direction];
        Cell::new_axial(self.q + direction.q, self.r + direction.r)
    }

    #[func(rename=get_neighbour_for_pointy_hex)]
    pub fn get_neighbour_for_pointy_hex_gd(&self, direction: DirectionPointy) -> Gd<Cell> {
        Gd::from_object(self.get_neighbour_for_pointy_hex(direction))
    }

    pub fn get_neighbour_for_flat_hex(&self, direction: DirectionFlat) -> Cell {
        let direction = CUBE_DIRECTIONS_FLAT[&direction];
        Cell::new_axial(self.q + direction.q, self.r + direction.r)
    }

    #[func(rename=get_neighbour_for_flat_hex)]
    pub fn get_neighbour_for_flat_hex_gd(&self, direction: DirectionFlat) -> Gd<Cell> {
        Gd::from_object(self.get_neighbour_for_flat_hex(direction))
    }
    
    #[func]
    fn equals(&self, other: Gd<Self>) -> bool {
        *self == *other.bind()
    }
}

static Q: &str = "Q";
static R: &str = "R";
static S: &str = "S";

static Q_PROPERTY_NAME: LazyLock<StringName> = LazyLock::new(|| StringName::from(Q));
static R_PROPERTY_NAME: LazyLock<StringName> = LazyLock::new(|| StringName::from(R));
static S_PROPERTY_NAME: LazyLock<StringName> = LazyLock::new(|| StringName::from(S));

#[godot_api]
impl IRefCounted for Cell {
    fn get_property(&self, property: StringName) -> Option<Variant> {
        if property == *Q_PROPERTY_NAME {
            Some(Variant::from(self.q))
        } else if property == *R_PROPERTY_NAME {
            Some(Variant::from(self.r))
        } else if property == *S_PROPERTY_NAME {
            Some(Variant::from(self.get_s()))
        } else {
            None
        }
    }

    fn set_property(&mut self, property: StringName, value: Variant) -> bool {
        if property == *Q_PROPERTY_NAME {
            self.q = value.to::<i32>();
            true
        } else if property == *R_PROPERTY_NAME {
            self.r = value.to::<i32>();
            true
        } else {
            false
        }
    }

    fn get_property_list(&mut self) -> Vec<PropertyInfo> {
        let mut s_info = PropertyInfo::new_export::<i32>(S);
        s_info.usage = PropertyUsageFlags::EDITOR | PropertyUsageFlags::READ_ONLY;
        vec![
            PropertyInfo::new_export::<i32>(Q),
            PropertyInfo::new_export::<i32>(R),
            s_info,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::DirectionPointy;

    macro_rules! new_axial_calculates_s_correctly {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (q, r, expected_s) = $value;
                let input = Cell::new_axial(q, r);
                assert_eq!(expected_s, input.get_s());
            }
        )*
        }
    }

    new_axial_calculates_s_correctly! {
        s_0: (0, 0, 0),
        s_1: (1, 0, -1),
        s_2: (1, 1, -2),
        s_3: (0, 1, -1),
        s_4: (-1, 0, 1),
        s_5: (-1, -1, 2),
        s_6: (0, -1, 1),
        s_7: (5, -2, -3),
        s_8: (2, -2, 0),
        s_9: (-9, 5, 4),
        s_10: (-9, -4, 13),
    }

    macro_rules! is_neighbour_returns_true_for_neighbour_positions {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let first = Cell::new_axial(0, 0);
                let second = $value;
                assert!(first.is_neighbour(&second));
            }
        )*
        }
    }

    is_neighbour_returns_true_for_neighbour_positions! {
        neighbour_top_left : Cell::new_axial(0, -1),
        neighbour_top_right :  Cell::new_axial(1, -1),
        neighbour_right :  Cell::new_axial(1, 0),
        neighbour_bottom_right :  Cell::new_axial(0, 1),
        neighbour_bottom_left :  Cell::new_axial(-1, 1),
        neighbour_left :  Cell::new_axial(-1, 0),
    }

    macro_rules! is_neighbour_returns_false_for_nonneighbour_positions {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let first = Cell::new_axial(0, 0);
                let second = $value;
                assert!(!first.is_neighbour(&second));
            }
        )*
        }
    }

    is_neighbour_returns_false_for_nonneighbour_positions! {
        non_neighbour_0 : Cell::new_axial(-1, -1),
        non_neighbour_1 : Cell::new_axial(-2, 0),
        non_neighbour_2 : Cell::new_axial(0, -2),
        non_neighbour_3 : Cell::new_axial(1, -2),
        non_neighbour_4 : Cell::new_axial(2, -2),
        non_neighbour_5 : Cell::new_axial(2, -1),
        non_neighbour_6 : Cell::new_axial(2, 0),
        non_neighbour_7 : Cell::new_axial(1, 1),
        non_neighbour_8 : Cell::new_axial(0, 2),
        non_neighbour_9 : Cell::new_axial(-1, 2),
        non_neighbour_10 : Cell::new_axial(-2, 2),
        non_neighbour_11 : Cell::new_axial(-2, 1),
        non_neighbour_12 : Cell::new_axial(-2, -5),
        non_neighbour_13 : Cell::new_axial(10, 0),
        non_neighbour_14 : Cell::new_axial(-5, 0),
        non_neighbour_16 : Cell::new_axial(0, 0),
        non_neighbour_17 : Cell::new_axial(10, -1),
        non_neighbour_18 : Cell::new_axial(-5, -1),
    }

    macro_rules! distance_to_returns_correct_distance {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (first, second, expected) = $value;
                assert_eq!(expected, first.distance_to(&second));
            }
        )*
        }
    }

    distance_to_returns_correct_distance! {
        distance_0: (Cell::new_axial(0, 0), Cell::new_axial(0, 0), 0),
        distance_1: (Cell::new_axial(0, 0), Cell::new_axial(1, 0), 1),
        distance_2: (Cell::new_axial(0, 0), Cell::new_axial(2, 0), 2),
        distance_3: (Cell::new_axial(0, 0), Cell::new_axial(5, 4), 9),
        distance_4: (Cell::new_axial(0, 0), Cell::new_axial(1, -5), 5),
        distance_5: (Cell::new_axial(0, 0), Cell::new_axial(-15, -5), 20),
        distance_6: (Cell::new_axial(0, 0), Cell::new_axial(30, -5), 30),
        distance_7: (Cell::new_axial(1, 0), Cell::new_axial(0, 0), 1),
        distance_8: (Cell::new_axial(1, 0), Cell::new_axial(5, 4), 8),
        distance_9: (Cell::new_axial(1, 4), Cell::new_axial(20, 9), 24),
        distance_10: (Cell::new_axial(20, 3), Cell::new_axial(-5, 4), 25),
        distance_11: (Cell::new_axial(-9, 13), Cell::new_axial(6, 31), 33),
    }

    #[test]
    fn zero_returns_position_with_q_r_s_at_0() {
        let position = Cell::zero();
        assert_eq!(0, position.q);
        assert_eq!(0, position.r);
        assert_eq!(0, position.get_s());
    }

    macro_rules! move_q_calculates_new_values_correctly {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (first, second, expected, s) = $value;
                let moved = first.move_q(second);
                assert_eq!(moved, expected);
                assert_eq!(moved.get_s(), s);
            }
        )*
        }
   }

    move_q_calculates_new_values_correctly! {
        move_q_0: (Cell::new_axial(0, 0), 0, Cell::new_axial(0, 0), 0),
        move_q_1: (Cell::new_axial(0, 0), 5, Cell::new_axial(5, -5), 0),
        move_q_2: (Cell::new_axial(0, 0), -10, Cell::new_axial(-10, 10), 0),
        move_q_3: (Cell::new_axial(5, 10), -7, Cell::new_axial(-2, 17), -15),
    }

    macro_rules! move_r_calculates_new_values_correctly {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (first, second, expected, s) = $value;
                let moved = first.move_r(second);
                assert_eq!(moved, expected);
                assert_eq!(moved.get_s(), s);
            }
        )*
        }
   }

    move_r_calculates_new_values_correctly! {
        move_r_0: (Cell::new_axial(0, 0), 0, Cell::new_axial(0, 0), 0),
        move_r_1: (Cell::new_axial(0, 0), 37, Cell::new_axial(0, 37), -37),
        move_r_2: (Cell::new_axial(0, 0), -15, Cell::new_axial(0, -15), 15),
        move_r_3: (Cell::new_axial(40, 5), -25, Cell::new_axial(40, -20), -20),
    }

    macro_rules! move_s_calculates_new_values_correctly {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (first, second, expected, s) = $value;
                let moved = first.move_s(second);
                assert_eq!(moved, expected);
                assert_eq!(moved.get_s(), s);
            }
        )*
        }
   }

    move_s_calculates_new_values_correctly! {
        move_s_0: (Cell::new_axial(0, 0), 0, Cell::new_axial(0, 0), 0),
        move_s_1: (Cell::new_axial(0, 0), 37, Cell::new_axial(-37, 0), 37),
        move_s_2: (Cell::new_axial(0, 0), -15, Cell::new_axial(15, 0), -15),
        move_s_3: (Cell::new_axial(12, 2), -3, Cell::new_axial(15, 2), -17),
    }

    macro_rules! get_neighbour_for_pointy_hex_returns_correct_values {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (hexagon, direction, expected) = $value;
                assert_eq!(hexagon.get_neighbour_for_pointy_hex(direction), expected);
            }
        )*
        }
    }

    get_neighbour_for_pointy_hex_returns_correct_values! {
        neighbour_pointy_ne: (Cell::new_axial(0, 0), DirectionPointy::TopRight, Cell::new_axial(1, -1)),
        neighbour_pointy_e: (Cell::new_axial(0, 0), DirectionPointy::Right, Cell::new_axial(1, 0)),
        neighbour_pointy_sw: (Cell::new_axial(0, 0), DirectionPointy::BottomLeft, Cell::new_axial(-1, 1)),
        neighbour_pointy_w: (Cell::new_axial(0, 0), DirectionPointy::Left, Cell::new_axial(-1, 0)),
        neighbour_pointy_se: (Cell::new_axial(0, 0), DirectionPointy::BottomRight, Cell::new_axial(0, 1)),
        neighbour_pointy_nw: (Cell::new_axial(0, 0), DirectionPointy::TopLeft, Cell::new_axial(0, -1)),
        neighbour_pointy_ne_2: (Cell::new_axial(1, 8), DirectionPointy::TopRight, Cell::new_axial(2, 7)),
        neighbour_pointy_e_2: (Cell::new_axial(5, 3), DirectionPointy::Right, Cell::new_axial(6, 3)),
        neighbour_pointy_se_2: (Cell::new_axial(-3, -8), DirectionPointy::BottomRight, Cell::new_axial(-3, -7)),
        neighbour_pointy_sw_2: (Cell::new_axial(-20, 13), DirectionPointy::BottomLeft, Cell::new_axial(-21, 14)),
        neighbour_pointy_w_2: (Cell::new_axial(6, -5), DirectionPointy::Left, Cell::new_axial(5, -5)),
        neighbour_pointy_nw_2: (Cell::new_axial(23, 42), DirectionPointy::TopLeft, Cell::new_axial(23, 41)),
    }

    macro_rules! get_neighbour_for_flat_hex_returns_correct_values {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (hexagon, direction, expected) = $value;
                assert_eq!(hexagon.get_neighbour_for_flat_hex(direction), expected);
            }
        )*
        }
    }

    get_neighbour_for_flat_hex_returns_correct_values! {
        neighbour_flat_n: (Cell::new_axial(0, 0), DirectionFlat::Top, Cell::new_axial(0, -1)),
        neighbour_flat_ne: (Cell::new_axial(0, 0), DirectionFlat::TopRight, Cell::new_axial(1, -1)),
        neighbour_flat_se: (Cell::new_axial(0, 0), DirectionFlat::BottomRight, Cell::new_axial(1, 0)),
        neighbour_flat_s: (Cell::new_axial(0, 0), DirectionFlat::Bottom, Cell::new_axial(0, 1)),
        neighbour_flat_sw: (Cell::new_axial(0, 0), DirectionFlat::BottomLeft, Cell::new_axial(-1, 1)),
        neighbour_flat_nw: (Cell::new_axial(0, 0), DirectionFlat::TopLeft, Cell::new_axial(-1, 0)),
        neighbour_flat_n_2: (Cell::new_axial(5, 3), DirectionFlat::Top, Cell::new_axial(5, 2)),
        neighbour_flat_ne_2: (Cell::new_axial(1, 8), DirectionFlat::TopRight, Cell::new_axial(2, 7)),
        neighbour_flat_se_2: (Cell::new_axial(-3, -8), DirectionFlat::BottomRight, Cell::new_axial(-2, -8)),
        neighbour_flat_s_2: (Cell::new_axial(6, -5), DirectionFlat::Bottom, Cell::new_axial(6, -4)),
        neighbour_flat_sw_2: (Cell::new_axial(-20, 13), DirectionFlat::BottomLeft, Cell::new_axial(-21, 14)),
        neighbour_flat_nw_2: (Cell::new_axial(23, 42), DirectionFlat::TopLeft, Cell::new_axial(22, 42)),
    }
}
