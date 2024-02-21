using System;
using Godot;
using Godot.Collections;

namespace Strategy;

[Tool]
public partial class HexagonCell : Node2D
{
    public Line2D Outline => GetNodeOrNull<Line2D>("Hexagon/Outline");
    public CanvasItem Attackable => GetNodeOrNull<CanvasItem>("Hexagon/Attackable");
    public CanvasItem Movable => GetNodeOrNull<CanvasItem>("Hexagon/Movable");
    public CanvasItem Selected => GetNodeOrNull<CanvasItem>("Hexagon/Selected");

    [Export]
    public Texture2D? TileTexture
    {
        get => GetNode<Sprite2D>("Tile").Texture;
        set => GetNode<Sprite2D>("Tile").Texture = value;
    }

    [Export]
    public Vector2 TileScale
    {
        get => GetNode<Sprite2D>("Tile").Scale;
        set => GetNode<Sprite2D>("Tile").Scale = value;
    }

    [Export]
    public float TileRotation
    {
        get => GetNode<Sprite2D>("Tile").Rotation;
        set => GetNode<Sprite2D>("Tile").Rotation = value;
    }

    /// <inheritdoc />
    public override void _Ready()
    {
        base._Ready();
    }
}
