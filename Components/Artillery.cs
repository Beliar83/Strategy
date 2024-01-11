using Godot;
using Strategy.FSharp;

namespace Strategy.Components;

[GlobalClass]
[Tool]
public partial class Artillery : Strategy.Component
{
    private readonly Systems.Artillery artillery = new();
    
    /// <inheritdoc />
    public override object? GetValue()
    {
        return artillery;
    }
}
