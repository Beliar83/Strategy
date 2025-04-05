using System;
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
    private bool syncColorWithPlayer = true;
    private Hexagon.Direction bodyDirection = Hexagon.Direction.North;
    private Hexagon.Direction weaponDirection = Hexagon.Direction.North;
    private bool isAttackIndirect;
    private Label? integrityLabel;
    private int integrity;

    [Export]
    public int Integrity
    {
        get => integrity;
        set
        {
            integrity = value;
            UpdateIntegrityLabel();
        }
    }

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
    public bool IsAttackIndirect
    {
        get => isAttackIndirect;
        set
        {
            isAttackIndirect = value;
            NotifyPropertyListChanged();
        }
    }

    [Export]
    public Hexagon.Direction BodyDirection
    {
        get => bodyDirection;
        set
        {
            bodyDirection = value;
            if (body is null) return;
            body.GlobalRotationDegrees = value switch
            {
                Hexagon.Direction.North => 0,
                Hexagon.Direction.NorthEast => 60,
                Hexagon.Direction.SouthEast => 120,
                Hexagon.Direction.South => 180,
                Hexagon.Direction.SouthWest => 240,
                Hexagon.Direction.NorthWest => 300,
                _ => throw new ArgumentOutOfRangeException(nameof(value)),
            };
        }
    }

    [Export]
    public Hexagon.Direction WeaponDirection
    {
        get => weaponDirection;
        set
        {
            weaponDirection = value;
            if (weapon is null) return;
            weapon.GlobalRotationDegrees = value switch
            {
                Hexagon.Direction.North => 0,
                Hexagon.Direction.NorthEast => 60,
                Hexagon.Direction.SouthEast => 120,
                Hexagon.Direction.South => 180,
                Hexagon.Direction.SouthWest => 240,
                Hexagon.Direction.NorthWest => 300,
                _ => throw new ArgumentOutOfRangeException(nameof(value)),
            };
        }
    }


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

    [Export]
    public float IndirectWeaponAngleDegrees { get; set; }
    
    [Export(PropertyHint.NodeType, nameof(Node2D))]
    public Node2D? Body
    {
        get => body;
        set
        {
            body = value;
            UpdateUnitColor();
            NotifyPropertyListChanged();
        }
    }

    [Export(PropertyHint.NodeType, nameof(Node2D))]
    public Node2D? Weapon
    {
        get => weapon;
        set
        {
            weapon = value;
            UpdateUnitColor();
            NotifyPropertyListChanged();
        }
    }

    [Export(PropertyHint.NodeType, nameof(Label))]
    public Label? IntegrityLabel
    {
        get => integrityLabel;
        set
        {
            integrityLabel = value;
            UpdateIntegrityLabel();
        }
    }

    private void UpdateIntegrityLabel()
    {
        if (IntegrityLabel is not null)
        {
            IntegrityLabel.Text = $"{Integrity}";
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

        if (gameWorld.PlayersByName.TryGetValue(player.PlayerId, out PlayerData? playerData))
        {
            Color = playerData.Color;
        }
        else
        {
            GD.PrintErr($"Player {player.PlayerId} not found");
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

        if (syncColorWithPlayer && propertyName == PropertyName.Color)
        {
            property["usage"] = (int)(PropertyUsageFlags.Default | PropertyUsageFlags.ReadOnly);
        }

        if (propertyName == PropertyName.WeaponDirection)
        {
            property["usage"] =
                weapon is null 
                ? (int)PropertyUsageFlags.None 
                : (int)PropertyUsageFlags.Default;
        }
        
        if (propertyName == PropertyName.BodyDirection)
        {
            property["usage"] = 
                body is null 
                ? (int)PropertyUsageFlags.None 
                : (int)PropertyUsageFlags.Default;
        }

        if (propertyName == PropertyName.IndirectWeaponAngleDegrees)
        {
            property["usage"] = 
                IsAttackIndirect 
                ? (int)PropertyUsageFlags.Default 
                : (int)PropertyUsageFlags.None;
        }
        
        base._ValidateProperty(property);
    }

    public bool IsInMovementRange(int distance) => distance > 0 && RemainingRange >= distance;
}
