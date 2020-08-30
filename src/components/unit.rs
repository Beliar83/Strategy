#[derive(Copy, Clone)]
pub struct Unit {
    pub integrity: i32,
    pub damage: i32,
    pub max_attack_range: i32,
    pub min_attack_range: i32,
    pub armor: i32,
    pub mobility: i32,
    pub remaining_range: i32,
    pub remaining_attacks: i32,
}

impl Unit {
    pub fn new(
        integrity: i32,
        damage: i32,
        max_attack_range: i32,
        min_attack_range: i32,
        armor: i32,
        mobility: i32,
        remaining_range: i32,
        remaining_attacks: i32,
    ) -> Unit {
        Unit {
            integrity,
            damage,
            max_attack_range,
            min_attack_range,
            armor,
            mobility,
            remaining_range,
            remaining_attacks,
        }
    }

    pub fn attack(&self, defender: &Unit) -> Result<AttackResult, AttackError> {
        if self.remaining_attacks <= 0 {
            Err(AttackError::NoAttacksLeft)
        } else {
            let actual_damage = self.damage - defender.armor;

            let mut attacker = self.clone();
            let mut defender = defender.clone();
            defender.integrity -= actual_damage;
            attacker.remaining_range = 0;
            attacker.remaining_attacks -= 1;
            Ok(AttackResult {
                actual_damage,
                attacker,
                defender,
            })
        }
    }

    pub fn can_move(&self, distance: i32) -> CanMove {
        if distance > 0 && self.remaining_range >= distance {
            CanMove::Yes(self.remaining_range - distance)
        } else {
            CanMove::No
        }
    }

    pub fn can_attack(&self, distance: i32) -> bool {
        distance <= self.max_attack_range && distance >= self.min_attack_range
    }
}

pub struct AttackResult {
    pub actual_damage: i32,
    pub attacker: Unit,
    pub defender: Unit,
}

pub enum AttackError {
    NoAttacksLeft,
}

pub enum CanMove {
    Yes(i32),
    No,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn attack_reduces_integrity() {
        let defender = Unit::new(5, 0, 0, 0, 0, 0, 0, 0);
        let attacker = Unit::new(0, 4, 0, 0, 0, 0, 0, 1);

        let result = attacker.attack(&defender);
        let result = match result {
            Ok(x) => x,
            Err(_) => panic!("Expected a result with Ok value"),
        };
        assert_eq!(result.defender.integrity, 1);
    }

    #[test]
    pub fn attack_sets_attacker_remaining_range_to_0() {
        let defender = Unit::new(5, 0, 0, 0, 0, 0, 0, 0);
        let attacker = Unit::new(0, 4, 0, 0, 0, 0, 5, 1);

        let result = attacker.attack(&defender);
        let result = match result {
            Ok(x) => x,
            Err(_) => panic!("Expected a result with Ok value"),
        };
        assert_eq!(result.attacker.remaining_range, 0);
    }

    #[test]
    pub fn attack_reduces_attacker_attacks_by_1() {
        let defender = Unit::new(5, 0, 0, 0, 0, 0, 0, 0);
        let attacker = Unit::new(0, 4, 0, 0, 0, 5, 0, 2);

        let result = attacker.attack(&defender);
        let result = match result {
            Ok(x) => x,
            Err(_) => panic!("Expected a result with Ok value"),
        };
        assert_eq!(result.attacker.remaining_attacks, 1);
        let attacker = result.attacker;
        let result = attacker.attack(&defender);
        let result = match result {
            Ok(x) => x,
            Err(_) => panic!("Expected a result with Ok value"),
        };
        assert_eq!(result.attacker.remaining_attacks, 0);
    }

    #[test]
    pub fn attack_takes_armor_into_account() {
        let defender = Unit::new(5, 0, 0, 0, 1, 0, 0, 0);
        let attacker = Unit::new(0, 4, 0, 0, 0, 0, 0, 1);

        let result = attacker.attack(&defender);
        let result = match result {
            Ok(x) => x,
            Err(_) => panic!("Expected a result with Ok value"),
        };
        assert_eq!(result.defender.integrity, 2);
    }

    #[test]
    pub fn attack_returns_correct_damage() {
        let defender = Unit::new(5, 0, 0, 0, 1, 0, 0, 0);
        let attacker = Unit::new(0, 4, 0, 0, 0, 0, 0, 1);

        let result = attacker.attack(&defender);
        let result = match result {
            Ok(x) => x,
            Err(_) => panic!("Expected a result with Ok value"),
        };
        assert_eq!(result.actual_damage, 3);
    }

    #[test]
    pub fn attack_returns_error_when_attacker_has_no_attack_left() {
        let defender = Unit::new(5, 0, 0, 0, 1, 0, 0, 0);
        let attacker = Unit::new(0, 4, 0, 0, 0, 0, 0, 0);

        let result = attacker.attack(&defender);
        match result {
            Ok(_) => panic!("Expected a result with Error value"),
            Err(_) => {}
        };
    }

    #[test]
    pub fn can_move_returns_ok_with_remaining_distance_if_distance_is_below_or_equal_to_remaining_range(
    ) {
        let unit = Unit::new(0, 0, 0, 0, 0, 0, 5, 0);
        let result = unit.can_move(4);
        match result {
            CanMove::Yes(remaining_range) => assert_eq!(remaining_range, 1),
            _ => panic!("Expected result of Yes"),
        }

        let result = unit.can_move(5);
        match result {
            CanMove::Yes(remaining_range) => assert_eq!(remaining_range, 0),
            _ => panic!("Expected result of Yes"),
        }
    }

    #[test]
    pub fn can_move_returns_no_if_distance_is_higher_than_remaining_range() {
        let unit = Unit::new(0, 0, 0, 0, 0, 0, 4, 0);
        let result = unit.can_move(5);
        match result {
            CanMove::No => {}
            _ => panic!("Expected result of No"),
        };
    }

    #[test]
    pub fn can_move_returns_no_if_distance_is_0() {
        let unit = Unit::new(0, 0, 0, 0, 0, 0, 4, 0);
        let result = unit.can_move(0);
        match result {
            CanMove::No => {}
            _ => panic!("Expected result of No"),
        };
    }

    #[test]
    pub fn can_attack_returns_true_if_distance_is_inside_range() {
        let unit = Unit::new(0, 0, 2, 1, 0, 0, 0, 0);
        assert!(unit.can_attack(1));
        assert!(unit.can_attack(2));
    }

    #[test]
    pub fn can_attack_returns_false_if_distance_is_outside_range() {
        let unit = Unit::new(0, 0, 2, 2, 0, 0, 0, 0);
        assert!(!unit.can_attack(3));
        assert!(!unit.can_attack(1));
    }
}
