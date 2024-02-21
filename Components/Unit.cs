using System;
using System.Diagnostics.CodeAnalysis;
using System.Linq;
using Godot;
using Godot.Collections;

namespace Strategy.Components;

[GlobalClass]
[Tool]
public partial class Unit : Component
{
    private static readonly Color DefaultColor = Colors.White;

    private Player? player;
    private Node2D? body;
    private Node2D? weapon;
    private Color color = DefaultColor;
    private bool syncColorWithPlayer;

    [Export]
    public int Integrity { get; set; }
    
    [Export]
    public int Damage { get; set; }
    
    [Export]
    public int MaxAttackRange { get; set; }
    
    [Export]
    public int MinAttackRange { get; set; }
    
    [Export]
    public int Armor { get; set; }
    
    [Export]
    public int Mobility { get; set; }
    
    [Export]
    public int RemainingRange { get; set; }
    
    [Export]
    public int RemainingAttacks { get; set; }

    [Export]
    public bool SyncColorWithPlayer
    {
        get => syncColorWithPlayer;
        set
        {
            syncColorWithPlayer = value;
            if (value)
            {
                SetColorToPlayerColor();
            }
            NotifyPropertyListChanged();
        }
    }

    [Export(PropertyHint.ColorNoAlpha)]
    public Color Color
    {
        get => color;
        set
        {
            SetField(ref color, value, PropertyName.Color);
            UpdateUnitColor();
        }
    }

    [Export(PropertyHint.NodeType, "Node2")]
    public Node2D? Body
    {
        get => body;
        set
        {
            body = value;
            UpdateUnitColor();
        }
    }

    [Export(PropertyHint.NodeType, "Node2")]
    public Node2D? Weapon
    {
        get => weapon;
        set
        {
            weapon = value;
            UpdateUnitColor();
        }
    }

    /// <inheritdoc />
    public override void _Notification(int what)
    {
        if (what == NotificationParented || what == NotificationUnparented)
        {
            Node? parent = GetParent();
            if (parent is not null)
            {
                parent.ChildEnteredTree += node =>
                {
                    // ReSharper disable once LocalVariableHidesMember
                    if (node is Player)
                    {
                        UpdatePlayer(parent);
                    }
                };

                parent.ChildExitingTree += node =>
                {
                    if (node is Player)
                    {
                        UpdatePlayer(parent);
                    }
                };
                UpdatePlayer(parent);
            }
            else
            {
                player = null;
            }
        }
        base._Notification(what);
        return;

        void UpdatePlayer(Node parent)
        {
            player = parent.GetChildren().OfType<Player>().FirstOrDefault();
            if (syncColorWithPlayer && player is not null)
            {
                player.PropertyChanged += (_, args) =>
                {
                    if (args.PropertyName == Player.PropertyName.PlayerId)
                    {
                        SetColorToPlayerColor();
                    }
                };
            }
        }
    }

    private void SetColorToPlayerColor()
    {
        if (gameWorld is null || player is null)
        {
            return;
        }

        PlayerData? playerData = gameWorld.Players.SingleOrDefault(p => p.Name == player.PlayerId);
        if (playerData is null)
        {
            GD.PrintErr($"Player {player.Name} not found");
        }
        else
        {
            Color = playerData.Color;
        }
    }

    public void UpdateUnitColor()
    {
        (Body?.Material as ShaderMaterial)?.SetShaderParameter("color", color);
        (Weapon?.Material as ShaderMaterial)?.SetShaderParameter("color", color);
    }

    /// <inheritdoc />
    public override void _ValidateProperty(Dictionary property)
    {
        StringName propertyName = property["name"].AsStringName();
        if (propertyName == PropertyName.SyncColorWithPlayer)
        {
            property["usage"] = (int)PropertyUsageFlags.Editor;
        }

        if (syncColorWithPlayer && propertyName == PropertyName.Color)
        {
            property["usage"] = (int)(PropertyUsageFlags.Default | PropertyUsageFlags.ReadOnly);
        }
        
        base._ValidateProperty(property);
    }
}
