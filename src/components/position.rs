mod position {
    use gdnative::prelude::*;

    pub fn is_adjacent(first: &Vector2, second: &Vector2) -> bool {
        // TODO: "There is probably an easier/faster way, but this can the refactored after the tests have been written

        ////////////
        // Hexagon//
        ////////////
        // ->/\<- //
        //->|  |<-//
        // ->\/<- //
        ////////////

        if first.x - second.x == 1.0 && first.y - second.y == 1.0 {
            // Top left : ->/\
            true
        } else if first.x - second.x == 0.0 && first.y - second.y == 1.0 {
            // Top right : /\<-
            true
        } else if first.x - second.x == 1.0 && first.y - second.y == 0.0 {
            // left : ->|  |
            true
        } else if first.x - second.x == -1.0 && first.y - second.y == 0.0 {
            // right : |  |<-
            true
        } else if first.x - second.x == 1.0 && first.y - second.y == -1.0 {
            // bottom left : ->\/
            true
        } else if first.x - second.x == 0.0 && first.y - second.y == -1.0 {
            // bottom right :\/<-
            true
        } else {
            false
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        pub fn is_adjacent_returns_true_for_adjacent_positions() {
            let first = Vector2::new(1.0, 1.0);
            let second = Vector2::new(0.0, 0.0); // Top left : ->/\
            assert!(is_adjacent(&first, &second));
            let second = Vector2::new(1.0, 0.0); // Top right : /\<-
            assert!(is_adjacent(&first, &second));
            let second = Vector2::new(0.0, 1.0); // left : ->|  |
            assert!(is_adjacent(&first, &second));
            let second = Vector2::new(2.0, 1.0); // right : |  |<-
            assert!(is_adjacent(&first, &second));
            let second = Vector2::new(0.0, 2.0); // bottom left : ->\/
            assert!(is_adjacent(&first, &second));
            let second = Vector2::new(1.0, 2.0); // bottom right :\/<-
            assert!(is_adjacent(&first, &second));
        }

        #[test]
        pub fn is_adjacent_returns_false_for_nonadjacent_positions() {
            todo!()
        }
    }
}
