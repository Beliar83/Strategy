/// Hexagonal map cube position as describe here: https://www.redblobgames.com/grids/hexagons/#coordinates-cube
pub struct Position {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

impl Position {
    pub fn zero() -> Self {
        Position { q: 0, r: 0, s: 0 }
    }

    /// Creates a position from axial coordinates
    pub fn new_axial(q: i32, r: i32) -> Self {
        // https://www.redblobgames.com/grids/hexagons/#conversions-axial
        Position { q, r, s: -q - r }
    }

    /// Creates a position from cube coordinates
    pub fn new_cube(q: i32, r: i32, s: i32) -> Self {
        Position { q, r, s }
    }

    pub fn distance_to(&self, other: &Position) -> i32 {
        // https://www.redblobgames.com/grids/hexagons/#distances-cube
        ((self.q - other.q).abs() + (self.r - other.r).abs() + (self.s - other.s).abs()) / 2
    }

    pub fn is_neighbour(&self, other: &Position) -> bool {
        self.distance_to(&other) == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! s_was_calculated_correctly {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, input.s);
            }
        )*
        }
    }

    s_was_calculated_correctly! {
        s_0: (Position::new_axial(0, 0), 0),
        s_1: (Position::new_axial(1, 0), -1),
        s_2: (Position::new_axial(1, 1), -2),
        s_3: (Position::new_axial(0, 1), -1),
        s_4: (Position::new_axial(-1, 0), 1),
        s_5: (Position::new_axial(-1, -1), 2),
        s_6: (Position::new_axial(0, -1), 1),
        s_7: (Position::new_axial(5, -2), -3),
        s_8: (Position::new_axial(2, -2), 0),
        s_9: (Position::new_axial(-9, 5), 4),
        s_10: (Position::new_axial(-9, -4), 13),
    }

    macro_rules! is_neighbour_returns_true_for_neighbour_positions {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let first = Position::new_axial(0, 0);
                let second = $value;
                assert!(first.is_neighbour(&second));
            }
        )*
        }
    }

    is_neighbour_returns_true_for_neighbour_positions! {
        neighbour_top_left : Position::new_axial(0, -1),
        neighbour_top_right :  Position::new_axial(1, -1),
        neighbour_right :  Position::new_axial(1, 0),
        neighbour_bottom_right :  Position::new_axial(0, 1),
        neighbour_bottom_left :  Position::new_axial(-1, 1),
        neighbour_left :  Position::new_axial(-1, 0),
    }

    macro_rules! is_neighbour_returns_false_for_nonneighbour_positions {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let first = Position::new_axial(0, 0);
                let second = $value;
                assert!(!first.is_neighbour(&second));
            }
        )*
        }
    }

    is_neighbour_returns_false_for_nonneighbour_positions! {
        non_neighbour_0 : Position::new_axial(-1, -1),
        non_neighbour_1 : Position::new_axial(-2, 0),
        non_neighbour_2 : Position::new_axial(0, -2),
        non_neighbour_3 : Position::new_axial(1, -2),
        non_neighbour_4 : Position::new_axial(2, -2),
        non_neighbour_5 : Position::new_axial(2, -1),
        non_neighbour_6 : Position::new_axial(2, 0),
        non_neighbour_7 : Position::new_axial(1, 1),
        non_neighbour_8 : Position::new_axial(0, 2),
        non_neighbour_9 : Position::new_axial(-1, 2),
        non_neighbour_10 : Position::new_axial(-2, 2),
        non_neighbour_11 : Position::new_axial(-2, 1),
        non_neighbour_12 : Position::new_axial(-2, -5),
        non_neighbour_13 : Position::new_axial(10, 0),
        non_neighbour_14 : Position::new_axial(-5, 0),
        non_neighbour_16 : Position::new_axial(0, 0),
        non_neighbour_17 : Position::new_axial(10, -1),
        non_neighbour_18 : Position::new_axial(-5, -1),
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
        distance_0: (Position::new_axial(0, 0), Position::new_axial(0, 0), 0),
        distance_1: (Position::new_axial(0, 0), Position::new_axial(1, 0), 1),
        distance_2: (Position::new_axial(0, 0), Position::new_axial(2, 0), 2),
        distance_3: (Position::new_axial(0, 0), Position::new_axial(5, 4), 9),
        distance_4: (Position::new_axial(0, 0), Position::new_axial(1, -5), 5),
        distance_5: (Position::new_axial(0, 0), Position::new_axial(-15, -5), 20),
        distance_6: (Position::new_axial(0, 0), Position::new_axial(30, -5), 30),
        distance_7: (Position::new_axial(1, 0), Position::new_axial(0, 0), 1),
        distance_8: (Position::new_axial(1, 0), Position::new_axial(5, 4), 8),
        distance_9: (Position::new_axial(1, 4), Position::new_axial(20, 9), 24),
        distance_10: (Position::new_axial(20, 3), Position::new_axial(-5, 4), 25),
        distance_11: (Position::new_axial(-9, 13), Position::new_axial(6, 31), 33),
    }

    #[test]
    fn zero_returns_postion_with_q_r_s_at_0() {
        let position = Position::zero();
        assert_eq!(0, position.q);
        assert_eq!(0, position.r);
        assert_eq!(0, position.s);
    }
}
