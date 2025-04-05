using Godot;

namespace Strategy.Components;

[GlobalClass]
[Tool]
public partial class Player : Component
{
    private StringName playerId = new();

    [Export]
    public StringName PlayerId
    {
        get => playerId;
        set => SetField(ref playerId, value, PropertyName.PlayerId);
    }
}
