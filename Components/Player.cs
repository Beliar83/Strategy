using System;
using Godot;

namespace Strategy.Components;

[GlobalClass]
[Tool]
public partial class Player : Component
{
    private StringName playerId = String.Empty;

    [Export]
    public StringName PlayerId
    {
        get => playerId;
        set => SetField(ref playerId, value, PropertyName.PlayerId);
    }
}
