/// Hexagonal map cube position as describe here: https://www.redblobgames.com/grids/hexagons/#coordinates-cube
pub struct Hexagon {
    pub q: i32,
    pub r: i32,
    pub s: i32,
    pub size: i32,
}

impl Hexagon {
    pub fn zero() -> Self {
        Hexagon {
            q: 0,
            r: 0,
            s: 0,
            size: 0,
        }
    }

    /// Creates a position from axial coordinates
    pub fn new_axial(q: i32, r: i32, size: i32) -> Self {
        // https://www.redblobgames.com/grids/hexagons/#conversions-axial
        Hexagon {
            q,
            r,
            s: -q - r,
            size: size,
        }
    }

    /// Creates a position from cube coordinates
    pub fn new_cube(q: i32, r: i32, s: i32, size: i32) -> Self {
        Hexagon { q, r, s, size }
    }

    pub fn distance_to(&self, other: &Hexagon) -> i32 {
        // https://www.redblobgames.com/grids/hexagons/#distances-cube
        ((self.q - other.q).abs() + (self.r - other.r).abs() + (self.s - other.s).abs()) / 2
    }

    pub fn is_neighbour(&self, other: &Hexagon) -> bool {
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
        s_0: (Hexagon::new_axial(0, 0, 0), 0),
        s_1: (Hexagon::new_axial(1, 0, 0), -1),
        s_2: (Hexagon::new_axial(1, 1, 0), -2),
        s_3: (Hexagon::new_axial(0, 1, 0), -1),
        s_4: (Hexagon::new_axial(-1, 0, 0), 1),
        s_5: (Hexagon::new_axial(-1, -1, 0), 2),
        s_6: (Hexagon::new_axial(0, -1, 0), 1),
        s_7: (Hexagon::new_axial(5, -2, 0), -3),
        s_8: (Hexagon::new_axial(2, -2, 0), 0),
        s_9: (Hexagon::new_axial(-9, 5, 0), 4),
        s_10: (Hexagon::new_axial(-9, -4, 0), 13),
    }

    macro_rules! is_neighbour_returns_true_for_neighbour_positions {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let first = Hexagon::new_axial(0, 0, 0);
                let second = $value;
                assert!(first.is_neighbour(&second));
            }
        )*
        }
    }

    is_neighbour_returns_true_for_neighbour_positions! {
        neighbour_top_left : Hexagon::new_axial(0, -1, 0),
        neighbour_top_right :  Hexagon::new_axial(1, -1, 0),
        neighbour_right :  Hexagon::new_axial(1, 0, 0),
        neighbour_bottom_right :  Hexagon::new_axial(0, 1, 0),
        neighbour_bottom_left :  Hexagon::new_axial(-1, 1, 0),
        neighbour_left :  Hexagon::new_axial(-1, 0, 0),
    }

    macro_rules! is_neighbour_returns_false_for_nonneighbour_positions {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let first = Hexagon::new_axial(0, 0, 0);
                let second = $value;
                assert!(!first.is_neighbour(&second));
            }
        )*
        }
    }

    is_neighbour_returns_false_for_nonneighbour_positions! {
        non_neighbour_0 : Hexagon::new_axial(-1, -1, 0),
        non_neighbour_1 : Hexagon::new_axial(-2, 0, 0),
        non_neighbour_2 : Hexagon::new_axial(0, -2, 0),
        non_neighbour_3 : Hexagon::new_axial(1, -2, 0),
        non_neighbour_4 : Hexagon::new_axial(2, -2, 0),
        non_neighbour_5 : Hexagon::new_axial(2, -1, 0),
        non_neighbour_6 : Hexagon::new_axial(2, 0, 0),
        non_neighbour_7 : Hexagon::new_axial(1, 1, 0),
        non_neighbour_8 : Hexagon::new_axial(0, 2, 0),
        non_neighbour_9 : Hexagon::new_axial(-1, 2, 0),
        non_neighbour_10 : Hexagon::new_axial(-2, 2, 0),
        non_neighbour_11 : Hexagon::new_axial(-2, 1, 0),
        non_neighbour_12 : Hexagon::new_axial(-2, -5, 0),
        non_neighbour_13 : Hexagon::new_axial(10, 0, 0),
        non_neighbour_14 : Hexagon::new_axial(-5, 0, 0),
        non_neighbour_16 : Hexagon::new_axial(0, 0, 0),
        non_neighbour_17 : Hexagon::new_axial(10, -1, 0),
        non_neighbour_18 : Hexagon::new_axial(-5, -1, 0),
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
        distance_0: (Hexagon::new_axial(0, 0, 0), Hexagon::new_axial(0, 0, 0), 0),
        distance_1: (Hexagon::new_axial(0, 0, 0), Hexagon::new_axial(1, 0, 0), 1),
        distance_2: (Hexagon::new_axial(0, 0, 0), Hexagon::new_axial(2, 0, 0), 2),
        distance_3: (Hexagon::new_axial(0, 0, 0), Hexagon::new_axial(5, 4, 0), 9),
        distance_4: (Hexagon::new_axial(0, 0, 0), Hexagon::new_axial(1, -5, 0), 5),
        distance_5: (Hexagon::new_axial(0, 0, 0), Hexagon::new_axial(-15, -5, 0), 20),
        distance_6: (Hexagon::new_axial(0, 0, 0), Hexagon::new_axial(30, -5, 0), 30),
        distance_7: (Hexagon::new_axial(1, 0, 0), Hexagon::new_axial(0, 0, 0), 1),
        distance_8: (Hexagon::new_axial(1, 0, 0), Hexagon::new_axial(5, 4, 0), 8),
        distance_9: (Hexagon::new_axial(1, 4, 0), Hexagon::new_axial(20, 9, 0), 24),
        distance_10: (Hexagon::new_axial(20, 3, 0), Hexagon::new_axial(-5, 4, 0), 25),
        distance_11: (Hexagon::new_axial(-9, 13, 0), Hexagon::new_axial(6, 31, 0), 33),
    }

    #[test]
    fn zero_returns_postion_with_q_r_s_at_0() {
        let position = Hexagon::zero();
        assert_eq!(0, position.q);
        assert_eq!(0, position.r);
        assert_eq!(0, position.s);
    }
}
