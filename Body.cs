using Godot;
using Godot.Collections;

namespace Strategy;

[Tool]
public partial class Body : FSharp.Body
{
    [Export((PropertyHint)35, "Node2D")]
    private new Array<NodePath> NodesWithColor
    {
        get => base.NodesWithColor;
        set => base.NodesWithColor = value;
    }

    /// <inheritdoc />
    public override void _Ready()
    {
        base._Ready();
    }
}
