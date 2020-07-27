#[derive(Copy, Clone)]
pub struct Unit {
    pub integrity: i32,
    pub damage: i32,
    pub armor: i32,
    pub mobility: i32,
}

impl Unit {
    pub fn new(integrity: i32, damage: i32, armor: i32, mobility: i32) -> Unit {
        Unit {
            integrity,
            damage,
            armor,
            mobility,
        }
    }
}

pub struct AttackResult {
    pub actual_damage: i32,
    pub remaining_integrity: i32,
}

pub fn attack(attacker: &Unit, defender: &mut Unit) -> AttackResult {
    let actual_damage = attacker.damage - defender.armor;
    defender.integrity -= actual_damage;
    AttackResult {
        actual_damage,
        remaining_integrity: defender.integrity,
    }
}

pub fn can_move(unit: &Unit, distance: i32) -> bool {
    unit.mobility >= distance
}

pub fn decrease_mobility(unit: &mut Unit, distance: i32) -> Result<i32, &str> {
    if can_move(unit, distance) {
        unit.mobility -= distance;
        Result::Ok(unit.mobility)
    } else {
        Result::Err("Could not use up mobility, distance too high.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn attack_reduces_integrity() {
        let mut defender = Unit::new(5, 0, 0, 0);
        let attacker = Unit::new(0, 4, 0, 0);

        attack(&attacker, &mut defender);
        assert_eq!(defender.integrity, 1);
    }

    #[test]
    pub fn attack_takes_armor_into_account() {
        let mut defender = Unit::new(5, 0, 1, 0);
        let attacker = Unit::new(0, 4, 0, 0);

        attack(&attacker, &mut defender);
        assert_eq!(defender.integrity, 2);
    }

    #[test]
    pub fn attack_returns_correct_result() {
        let mut defender = Unit::new(5, 0, 1, 0);
        let attacker = Unit::new(0, 4, 0, 0);

        let result: AttackResult = attack(&attacker, &mut defender);
        assert_eq!(result.actual_damage, 3);
        assert_eq!(result.remaining_integrity, 2);
    }

    #[test]
    pub fn can_move_returns_true_if_distance_below_or_equal_to_mobility() {
        let unit = Unit::new(0, 0, 0, 5);
        assert!(can_move(&unit, 4));
        assert!(can_move(&unit, 5));
    }

    #[test]
    pub fn decrease_mobility_decreases_mobility_and_returns_remaining_mobility() {
        let mut unit = Unit::new(0, 0, 0, 5);
        let remaining = decrease_mobility(&mut unit, 4).unwrap();
        assert_eq!(unit.mobility, 1);
        assert_eq!(unit.mobility, remaining);
        decrease_mobility(&mut unit, 1).unwrap();
        assert_eq!(unit.mobility, 0);
    }

    #[test]
    pub fn decrease_mobility_does_not_change_mobilty_if_distance_is_greater_than_mobility() {
        let mut unit = Unit::new(0, 0, 0, 5);
        decrease_mobility(&mut unit, 6).unwrap_err();
        assert_eq!(unit.mobility, 5);
    }

    #[test]
    pub fn decrease_mobility_returns_error_if_distance_is_greater_than_mobility() {
        let mut unit = Unit::new(0, 0, 0, 5);
        let value = decrease_mobility(&mut unit, 6);
        assert_eq!(
            value,
            Result::Err("Could not use up mobility, distance too high.")
        );
    }
}
