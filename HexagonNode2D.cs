using System.Collections.Generic;
using Godot;
using Godot.Collections;
using Array = System.Array;

namespace Strategy;

[GlobalClass]
[Tool]
public partial class HexagonNode2D : Node2D
{
    private TileMap? tileMap;
    public Hexagon Hexagon { get; set; } = Hexagon.Zero;

    [Export]
    [ExportCategory("Hexagon")]
    public int Q
    {
        get => Hexagon.Q;
        set
        {
            Hexagon.Q = value;
            UpdatePosition();
        }
    }

    [Export]
    [ExportCategory("Hexagon")]
    public int R
    {
        get => Hexagon.R;
        set
        {
            Hexagon.R = value;
            UpdatePosition();
        }
    }

    [Export]
    [ExportCategory("Hexagon")]
    public int S
    {
        get => Hexagon.S;
        set
        {
            Hexagon.S = value; 
            UpdatePosition();
        }
    }

    /// <inheritdoc />
    public override void _Ready()
    {
        UpdatePosition();
        base._Ready();
    }

    /// <inheritdoc />
    public override void _ValidateProperty(Dictionary property)
    {
        StringName propertyName = property["name"].AsStringName();
        if (propertyName == Node2D.PropertyName.Position)
        {
            property["usage"] = (int)(PropertyUsageFlags.Editor | PropertyUsageFlags.ReadOnly);
        }

        if (propertyName == PropertyName.S)
        {
            property["usage"] = (int)PropertyUsageFlags.Editor;
        }
        
        if (tileMap is null && (propertyName == PropertyName.Q || propertyName == PropertyName.R))
        {
            property["usage"] = (int)(PropertyUsageFlags.Default | PropertyUsageFlags.ReadOnly);
        }
        base._ValidateProperty(property);
    }

    public void UpdatePosition()
    {
        if (tileMap is null || !IsNodeReady()) return;

        // Position = tileMap.MapToLocal(new Vector2I(Hexagon.Q, Hexagon.R));
        // Position = Hexagon.Get2DPosition(tileMap.TileSet.TileSize.X / 2.0f);
        // Position = tileMap.MapToLocal(Hexagon.GetOffsetPosition());
        GD.Print(Hexagon.Get2DPosition(tileMap.TileSet.TileSize.X));
        GD.Print(tileMap.MapToLocal(new Vector2I(Hexagon.Q, Hexagon.R)));
        GD.Print(tileMap.MapToLocal(Hexagon.GetOffsetPosition()));
        Position = tileMap.MapToLocal(Hexagon.GetOffsetPosition());
        
        // Position = Hexagon.Get2DPosition(tileMap.TileSet.TileSize.X) - new Vector2I(tileMap.TileSet.TileSize.X / 4, 0);
    }

    /// <inheritdoc />
    public override void _Notification(int what)
    {
        if (what != NotificationParented)
        {
            return;
        }

        tileMap = GetParentOrNull<TileMap>();
        UpdatePosition();
    }

    /// <inheritdoc />
    public override string[] _GetConfigurationWarnings()
    {
        return tileMap is null ? 
            new[] { "Node needs to be a direct child of a TileMap" } :
            Array.Empty<string>();
    }
}
