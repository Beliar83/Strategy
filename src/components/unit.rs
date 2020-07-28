use legion::prelude::*;

#[derive(Copy, Clone)]
pub struct Unit {
    pub integrity: i32,
    pub damage: i32,
    pub armor: i32,
    pub mobility: i32,
    pub remaining_range: i32,
}

impl Unit {
    pub fn new(
        integrity: i32,
        damage: i32,
        armor: i32,
        mobility: i32,
        remaining_range: i32,
    ) -> Unit {
        Unit {
            integrity,
            damage,
            armor,
            mobility,
            remaining_range,
        }
    }

    pub fn attack(&self, defender: &Unit) -> AttackResult {
        let actual_damage = self.damage - defender.armor;

        let mut attacker = self.clone();
        let mut defender = defender.clone();
        defender.integrity -= actual_damage;
        attacker.mobility = 0;
        AttackResult {
            actual_damage,
            attacker,
            defender,
        }
    }

    pub fn can_move(&self, distance: i32) -> CanMove {
        if self.remaining_range >= distance {
            CanMove::Yes(self.remaining_range - distance)
        } else {
            CanMove::No
        }
    }
}

pub struct AttackResult {
    pub actual_damage: i32,
    pub attacker: Unit,
    pub defender: Unit,
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
        let defender = Unit::new(5, 0, 0, 0, 0);
        let attacker = Unit::new(0, 4, 0, 0, 0);

        let result = attacker.attack(&defender);
        assert_eq!(result.defender.integrity, 1);
    }

    #[test]
    pub fn attack_reduces_sets_mobility_to_0() {
        let defender = Unit::new(5, 0, 0, 0, 0);
        let attacker = Unit::new(0, 4, 0, 5, 0);

        let result: AttackResult = attacker.attack(&defender);
        assert_eq!(result.attacker.mobility, 0);
    }

    #[test]
    pub fn attack_takes_armor_into_account() {
        let defender = Unit::new(5, 0, 1, 0, 0);
        let attacker = Unit::new(0, 4, 0, 0, 0);

        let result: AttackResult = attacker.attack(&defender);
        assert_eq!(result.defender.integrity, 2);
    }

    #[test]
    pub fn attack_returns_correct_damage() {
        let defender = Unit::new(5, 0, 1, 0, 0);
        let attacker = Unit::new(0, 4, 0, 0, 0);

        let result: AttackResult = attacker.attack(&defender);
        assert_eq!(result.actual_damage, 3);
    }

    #[test]
    pub fn can_move_returns_ok_with_remaining_distance_if_distance_is_below_or_equal_to_remaining_range(
    ) {
        let mut unit = Unit::new(0, 0, 0, 0, 5);
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
        let mut unit = Unit::new(0, 0, 0, 0, 4);
        let result = unit.can_move(5);
        match result {
            CanMove::No => {}
            _ => panic!("Expected result of No"),
        };
    }
}
