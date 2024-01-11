using Godot;
using Strategy.FSharp;

namespace Strategy.Components;

[GlobalClass]
[Tool]
public partial class Tank : Component
{
    private readonly Systems.Tank tank = new();

    /// <inheritdoc />
    public override object? GetValue()
    {
        return tank;
    }
}
