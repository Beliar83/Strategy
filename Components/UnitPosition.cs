using Godot;
using Strategy.FSharp;

namespace Strategy.Components;

[GlobalClass]
[Tool]
public partial class UnitPosition : Component
{
    private Systems.UnitPosition unitPosition = new(FSharp.Hexagon.Hexagon.Zero, 0, 0);

    private Hexagon hexagon = new();

    public UnitPosition()
    {
        UpdatePropertyChangedHandler();
    }

    [Export]
    public Hexagon Hexagon
    {
        get => hexagon;
        set
        {
            hexagon = value;
            UpdatePropertyChangedHandler();

            unitPosition = new Systems.UnitPosition(value.InternalValue, unitPosition.BodyRotation,
                unitPosition.WeaponRotation);
            Entity?.UpdateComponent(unitPosition);
        }
    }

    private void UpdatePropertyChangedHandler()
    {
        hexagon.PropertyChanged += (_, args) =>
        {
            if (args.PropertyName != nameof(Strategy.Hexagon.InternalValue))
            {
                return;
            }

            unitPosition = new Systems.UnitPosition(hexagon.InternalValue, unitPosition.BodyRotation,
                unitPosition.WeaponRotation);
            Entity?.UpdateComponent(unitPosition);
        };
    }

    [Export]
    public float BodyRotation
    {
        get => unitPosition.BodyRotation;
        set
        {
            unitPosition = new Systems.UnitPosition(unitPosition.Position, value, unitPosition.WeaponRotation);
            Entity?.UpdateComponent(unitPosition);
        }
    }

    [Export]
    public float WeaponRotation
    {
        get => unitPosition.WeaponRotation;
        set
        {
            unitPosition = new Systems.UnitPosition(unitPosition.Position, unitPosition.BodyRotation, value);
            Entity?.UpdateComponent(unitPosition);
        }
    }

    /// <inheritdoc />
    public override object? GetValue()
    {
        return unitPosition;
    }
}
