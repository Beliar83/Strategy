use gdnative::core_types::Vector2;
use std::hash::Hash;

pub const CELL_SIZE: f32 = 40.0;

/// Hexagonal map cube position as describe here: https://www.redblobgames.com/grids/hexagons/#coordinates-cube
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Cell {
    q: i32,
    r: i32,
    s: i32,
}

impl Cell {
    pub fn zero() -> Self {
        Cell { q: 0, r: 0, s: 0 }
    }

    /// Creates a position from axial coordinates
    pub fn new_axial(q: i32, r: i32) -> Self {
        // https://www.redblobgames.com/grids/hexagons/#conversions-axial
        Cell {
            q,
            r,
            s: calculate_axis(q, r),
        }
    }

    pub fn at_2d_position(pos: Vector2) -> Cell {
        let q = (3_f32.sqrt() / 3_f32 * pos.x - 1_f32 / 3_f32 * pos.y) / (CELL_SIZE);
        let r = (2_f32 / 3_f32 * pos.y) / (CELL_SIZE);
        let s = -q - r;

        cube_round(q, r, s)
    }

    pub fn get_2d_position(&self) -> Vector2 {
        let x = CELL_SIZE
            * (3.0_f32.sqrt() * (self.get_q() as f32)
                + 3.0_f32.sqrt() / 2.0 * (self.get_r() as f32));
        let y = CELL_SIZE * (3.0 / 2.0 * (self.get_r() as f32));
        Vector2::new(x, y)
    }

    // Create from Vector2 representation for easy passing to and from Godot
    pub fn from_vector2(vector: Vector2) -> Cell {
        let q = vector.x as i32;
        let r = vector.y as i32;
        Cell::new_axial(q, r)
    }

    // Represent as Vector2 for easy passing to and from Godot
    pub fn as_vector2(&self) -> Vector2 {
        let x = self.q as f32;
        let y = self.r as f32;
        Vector2::new(x, y)
    }

    pub fn move_q(&self, length: i32) -> Cell {
        Self::new_cube(self.q + length, self.r, self.s - length)
    }

    pub fn move_r(&self, length: i32) -> Cell {
        Self::new_cube(self.q, self.r + length, self.s - length)
    }

    pub fn move_s(&self, length: i32) -> Cell {
        Self::new_cube(self.q - length, self.r + length, self.s)
    }

    /// Creates a position from cube coordinates
    pub fn new_cube(q: i32, r: i32, s: i32) -> Self {
        Cell { q, r, s }
    }

    pub fn get_q(&self) -> i32 {
        self.q
    }

    pub fn get_r(&self) -> i32 {
        self.r
    }

    pub fn get_s(&self) -> i32 {
        self.s
    }

    pub fn distance_to(&self, other: &Cell) -> i32 {
        // https://www.redblobgames.com/grids/hexagons/#distances-cube
        ((self.q - other.q).abs() + (self.r - other.r).abs() + (self.s - other.s).abs()) / 2
    }

    pub fn is_neighbour(&self, other: &Cell) -> bool {
        self.distance_to(&other) == 1
    }

    pub fn get_neighbour(&self, direction: Direction) -> Cell {
        let cube_directions = [
            Cell::new_cube(1, 0, -1),
            Cell::new_cube(1, -1, 0),
            Cell::new_cube(0, -1, 1),
            Cell::new_cube(-1, 0, 1),
            Cell::new_cube(-1, 1, 0),
            Cell::new_cube(0, 1, -1),
        ];
        let direction = cube_directions[direction as usize];
        Cell::new_cube(
            self.q + direction.q,
            self.r + direction.r,
            self.s + direction.s,
        )
    }
}

fn calculate_axis(axis_1: i32, axis_2: i32) -> i32 {
    -axis_1 - axis_2
}

fn cube_round(x: f32, y: f32, z: f32) -> Cell {
    let mut rx = x.round();
    let mut ry = y.round();
    let mut rz = z.round();

    let x_diff = (rx - x).abs();
    let y_diff = (ry - y).abs();
    let z_diff = (rz - z).abs();

    if (x_diff > y_diff) & (x_diff > z_diff) {
        rx = -ry - rz
    } else if y_diff > z_diff {
        ry = -rx - rz
    } else {
        rz = -rx - ry
    }
    Cell::new_cube(rx as i32, ry as i32, rz as i32)
}

pub enum Direction {
    East = 0,
    NorthEast = 1,
    NorthWest = 2,
    West = 3,
    SouthWest = 4,
    SouthEast = 5,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::cell::Direction::{
        East, NorthEast, NorthWest, SouthEast, SouthWest, West,
    };

    macro_rules! new_axial_calculates_s_correctly {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (q, r, expected) = $value;
                let input = Hexagon::new_axial(q, r);
                assert_eq!(expected, input.s);
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
                let first = Hexagon::new_axial(0, 0);
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
                let first = Hexagon::new_axial(0, 0);
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
    fn zero_returns_postion_with_q_r_s_at_0() {
        let position = Cell::zero();
        assert_eq!(0, position.q);
        assert_eq!(0, position.r);
        assert_eq!(0, position.s);
    }

    macro_rules! move_q_calculates_new_values_correctly {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (first, second, expected) = $value;
                assert_eq!(first.move_q(second), expected);
            }
        )*
        }
   }

    move_q_calculates_new_values_correctly! {
        move_q_0: (Cell::new_axial(0, 0), 0, Cell::new_cube(0, 0, 0)),
        move_q_1: (Cell::new_axial(0, 0), 5, Cell::new_cube(5, 0, -5)),
        move_q_2: (Cell::new_axial(0, 0), -10, Cell::new_cube(-10, 0, 10)),
        move_q_3: (Cell::new_axial(5, 10), -7, Cell::new_cube(-2, 10, -8)),
    }

    macro_rules! move_r_calculates_new_values_correctly {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (first, second, expected) = $value;
                assert_eq!(first.move_r(second), expected);
            }
        )*
        }
   }

    move_r_calculates_new_values_correctly! {
        move_r_0: (Cell::new_axial(0, 0), 0, Cell::new_cube(0, 0, 0)),
        move_r_1: (Cell::new_axial(0, 0), 37, Cell::new_cube(0, 37, -37)),
        move_r_2: (Cell::new_axial(0, 0), -15, Cell::new_cube(0, -15, 15)),
        move_r_3: (Cell::new_axial(40, 5), -25, Cell::new_cube(40, -20, -20)),
    }

    macro_rules! move_s_calculates_new_values_correctly {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (first, second, expected) = $value;
                assert_eq!(first.move_s(second), expected);
            }
        )*
        }
   }

    move_s_calculates_new_values_correctly! {
        move_s_0: (Cell::new_axial(0, 0), 0, Cell::new_cube(0, 0, 0)),
        move_s_1: (Cell::new_axial(0, 0), 37, Cell::new_cube(-37, 37, 0)),
        move_s_2: (Cell::new_axial(0, 0), -15, Cell::new_cube(15, -15, 0)),
        move_s_3: (Cell::new_axial(12, 2), -3, Cell::new_cube(15, -1, -14)),
    }

    macro_rules! get_neighbour_returns_correct_values {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (hexagon, direction, expected) = $value;
                assert_eq!(hexagon.get_neighbour(direction), expected);
            }
        )*
        }
    }

    get_neighbour_returns_correct_values! {
        neighbour_e: (Cell::new_axial(0, 0), East, Cell::new_cube(1, 0, -1)),
        neighbour_ne: (Cell::new_axial(0, 0), NorthEast, Cell::new_cube(1, -1, 0)),
        neighbour_nw: (Cell::new_axial(0, 0), NorthWest, Cell::new_cube(0, -1, 1)),
        neighbour_w: (Cell::new_axial(0, 0), West, Cell::new_cube(-1, 0, 1)),
        neighbour_sw: (Cell::new_axial(0, 0), SouthWest, Cell::new_cube(-1, 1, 0)),
        neighbour_se: (Cell::new_axial(0, 0), SouthEast, Cell::new_cube(0, 1, -1)),
        neighbour_e_2: (Cell::new_axial(5, 3), East, Cell::new_cube(6, 3, -9)),
        neighbour_ne_2: (Cell::new_axial(1, 8), NorthEast, Cell::new_cube(2, 7, -9)),
        neighbour_nw_2: (Cell::new_axial(23, 42), NorthWest, Cell::new_cube(23, 41, -64)),
        neighbour_w_2: (Cell::new_axial(6, -5), West, Cell::new_cube(5, -5, 0)),
        neighbour_sw_2: (Cell::new_axial(-20, 13), SouthWest, Cell::new_cube(-21, 14, 7)),
        neighbour_se_2: (Cell::new_axial(-3, -8), SouthEast, Cell::new_cube(-3, -7, 10)),
    }
}
