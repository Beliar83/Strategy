using Godot;

namespace Strategy;


public partial class MenuNode(string text) : SubViewport
{
    /// <inheritdoc />
    public override void _Ready()
    {
        TransparentBg = true;
        Label label = new() { Text = text };
        label.Resized += () =>
        {
            Size = (Vector2I)label.Size;
        };
        
        AddChild(label);
        base._Ready();
    }
}
