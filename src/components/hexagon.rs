/// Hexagonal map cube position as describe here: https://www.redblobgames.com/grids/hexagons/#coordinates-cube
#[derive(Copy, Clone)]
pub struct Hexagon {
    q: i32,
    r: i32,
    s: i32,
    size: i32,
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
        let mut hexagon = Hexagon { q, r, s: 0, size };
        hexagon.update_s();
        hexagon
    }

    fn update_s(&mut self) {
        self.s = -self.r - self.q;
    }

    /// Creates a position from cube coordinates
    pub fn new_cube(q: i32, r: i32, s: i32, size: i32) -> Self {
        Hexagon { q, r, s, size }
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

    pub fn get_size(&self) -> i32 {
        self.size
    }

    pub fn set_size(&mut self, size: i32) {
        self.size = size
    }

    pub fn set_axial(&mut self, q: i32, r: i32) {
        self.q = q;
        self.r = r;
        self.update_s();
    }

    pub fn set_cube(&mut self, q: i32, r: i32, s: i32) {
        self.q = q;
        self.r = r;
        self.s = s;
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

    macro_rules! new_axial_calculates_s_correctly {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (q, r, expected) = $value;
                let input = Hexagon::new_axial(q, r, 0);
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

    macro_rules! set_axial_calculates_s_correctly {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (q, r, expected) = $value;
                let mut input = Hexagon::new_axial(0, 0, 0);
                input.set_axial(q, r);
                assert_eq!(expected, input.s);
            }
        )*
        }
    }

    set_axial_calculates_s_correctly! {
        set_s_0: (0, 0, 0),
        set_s_1: (1, 0, -1),
        set_s_2: (1, 1, -2),
        set_s_3: (0, 1, -1),
        set_s_4: (-1, 0, 1),
        set_s_5: (-1, -1, 2),
        set_s_6: (0, -1, 1),
        set_s_7: (5, -2, -3),
        set_s_8: (2, -2, 0),
        set_s_9: (-9, 5, 4),
        set_s_10: (-9, -4, 13),
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
