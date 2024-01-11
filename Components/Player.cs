using System.Collections.Generic;
using Godot;
using Godot.Collections;

namespace Strategy.Components;

[GlobalClass]
[Tool]
public partial class Player : Component
{
    private static readonly StringName PlayerIdName = new("PlayerId");
    
    private Strategy.FSharp.Player.Player? player;

    /// <inheritdoc />
    public override Array<Dictionary> _GetPropertyList()
    {
        Array<Dictionary> properties = base._GetPropertyList() ?? new Array<Dictionary>();
        var propertyData = new Dictionary();
        propertyData["name"] = PlayerIdName;
        propertyData["type"] = (int)Variant.Type.String;
        propertyData["usage"] = (int)PropertyUsageFlags.Default;
        propertyData["hint"] = (int)PropertyHint.Enum;
        List<string> playerNames = Entity?.GameWorld?.GetPlayerNames() ?? new List<string>();
        propertyData["hint_string"] = string.Join(',', playerNames);
        properties.Add(propertyData);
        return properties;
    }

    /// <inheritdoc />
    public override bool _Set(StringName property, Variant value)
    {
        if (property == PlayerIdName)
        {
            player = new FSharp.Player.Player(value.AsString());
            Entity?.UpdateComponent(player);
            return true;

        }
        
        return base._Set(property, value);
    }

    /// <inheritdoc />
    public override Variant _Get(StringName property)
    {
        if (property == PlayerIdName)
        {
            return player?.PlayerId ?? "";
        }

        return base._Get(property);
    }

    /// <inheritdoc />
    public override object? GetValue()
    {
        return player;
    }
}
