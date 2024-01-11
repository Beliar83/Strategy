using Godot;
using Strategy.FSharp;

namespace Strategy.Components;

[GlobalClass]
[Tool]
public partial class Unit : Component
{
    private Strategy.FSharp.Systems.Unit unit = new(0, 0, 0, 0, 0, 0, 0, 0);
    
    [Export]
    public int Integrity
    {
        get => unit.Integrity;
        set
        {
            unit = new Systems.Unit(value, unit.Damage, unit.MaxAttackRange,  unit.MinAttackRange, unit.Armor, unit.Mobility, 0, 0);
            Entity?.UpdateComponent(unit);
        }
    }
    
    [Export]
    public int Damage
    {
        get => unit.Damage;
        set
        {
            unit = new Systems.Unit(unit.Integrity, value, unit.MaxAttackRange,  unit.MinAttackRange, unit.Armor, unit.Mobility, 0, 0);
            Entity?.UpdateComponent(unit);
        }
    }
        
    [Export]
    public int MaxAttackRange
    {
        get => unit.MaxAttackRange;
        set
        {
            unit = new Systems.Unit(unit.Integrity, unit.Damage, value,  unit.MinAttackRange, unit.Armor, unit.Mobility, 0, 0);
            Entity?.UpdateComponent(unit);
        }
    }
        
    [Export]
    public int MinAttackRange
    {
        get => unit.MinAttackRange;
        set
        {
            unit = new Systems.Unit(unit.Integrity, unit.Damage, unit.MaxAttackRange, value, unit.Armor, unit.Mobility, 0, 0);
            Entity?.UpdateComponent(unit);
        }
    }

    [Export]
    public int Armor
    {
        get => unit.Armor;
        set
        {
            unit = new Systems.Unit(unit.Integrity, unit.Damage, unit.MaxAttackRange, unit.MinAttackRange, value, unit.Mobility, 0, 0);
            Entity?.UpdateComponent(unit);
        }
    }

    [Export]
    public int Mobility
    {
        get => unit.Mobility;
        set
        {
            unit = new Systems.Unit(unit.Integrity, unit.Damage, unit.MaxAttackRange, unit.MinAttackRange, unit.Armor, value, 0, 0);
            Entity?.UpdateComponent(unit);
        }
    }
    
    /// <inheritdoc />
    public override object? GetValue()
    {
        return unit;
    }
}
