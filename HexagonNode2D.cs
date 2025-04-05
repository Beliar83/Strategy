using Godot;
using Godot.Collections;

namespace Strategy;

[GlobalClass]
[Tool]
public partial class HexagonNode2D : Node2D
{
    private TileMapLayer? tileMap;
    
    public Hexagon Hexagon { get; set; } = Hexagon.Zero;
    
    [ExportCategory("Hexagon")]
    [Export]
    public int Q
    {
        get => Hexagon.Q;
        set
        {
            Hexagon = Hexagon.MoveQ(value - Hexagon.Q);
            UpdatePosition();
        }
    }

    [Export]
    public int R
    {
        get => Hexagon.R;
        set
        {
            Hexagon = Hexagon.MoveR(value - Hexagon.R);
            UpdatePosition();
        }
    }

    [Export]
    public int S
    {
        get => Hexagon.S;
        set
        {
            Hexagon = Hexagon.MoveS(value - Hexagon.S);
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

        if (tileMap is null && (propertyName == PropertyName.Q || propertyName == PropertyName.R))
        {
            property["usage"] = (int)(PropertyUsageFlags.Default | PropertyUsageFlags.ReadOnly);
        }
        base._ValidateProperty(property);
    }

    public void UpdatePosition()
    {
        if (tileMap is null || !IsNodeReady()) return;

        Position = tileMap.MapToLocal(Hexagon.GetOffsetPosition());
    }

    /// <inheritdoc />
    public override void _Notification(int what)
    {
        if (what != NotificationParented)
        {
            return;
        }

        tileMap = GetParentOrNull<HexMap>();
        UpdatePosition();
    }

    /// <inheritdoc />
    public override string[] _GetConfigurationWarnings()
    {
        return tileMap is null 
            ? ["Node needs to be a direct child of a TileMapLayer"]
            : [];
    }
}
