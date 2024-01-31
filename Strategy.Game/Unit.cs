namespace Strategy.Game;

public record struct Unit(
    int Integrity,
    int Damage,
    int MaxAttackRange,
    int MinAttackRange,
    int Armor,
    int Mobility,
    int RemainingRange,
    int RemainingAttacks);
