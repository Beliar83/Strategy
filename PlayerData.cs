using System.Linq;
using Godot;

namespace Strategy;

[GlobalClass]
[Tool]
public partial class PlayerData : Resource
{
    private Color color = new(0, 0, 0);
    private StringName name = "Player";

    public GameWorld? GameWorld { get; set; }

    [Export]
    public StringName Name
    {
        get => name;
        set
        {
            if (GameWorld?.Players.All(p => p.name != value) ?? true)
            {
                name = value;
                if (Engine.IsEditorHint())
                {
                    GameWorld?.PlayerChanged();
                }
            }
        }
    }

    [Export(PropertyHint.ColorNoAlpha)]
    public Color Color
    {
        get => color;
        set => color = value;
    }
}
