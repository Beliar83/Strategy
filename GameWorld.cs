using System;
using System.Collections.Generic;
using System.Linq;
using Garnet.Composition;
using Godot;
using Godot.Collections;
using Microsoft.FSharp.Collections;
using Strategy.FSharp;

namespace Strategy;

[Tool]
public partial class GameWorld : FSharp.GameWorld.GameWorld
{
    private static readonly StringName EntitiesName = new("Entities");

    private Array<Entity> entities = new();

    private Array<PlayerData> players = new();

    public GameWorld()
    {
        SyncPlayers(new List<Tuple<string, Player.PlayerData>>());
    }

    /// <inheritdoc />
    public override void _Ready()
    {
        base._Ready();
    }

    /// <inheritdoc />
    public override void _Process(double delta)
    {
        base._Process(delta);
        if (Engine.Singleton.IsEditorHint())
        {
            foreach (Entity entity in entities)
            {
                SetComponents(entity.Id,
                    ListModule.OfSeq(entity.Components.Where(c => c is not null).Select(c => c.GetValue())
                        .Where(v => v is not null)));
            }
        }
    }

    /// <inheritdoc />
    public override void _PhysicsProcess(double delta)
    {
        base._PhysicsProcess(delta);
    }

    /// <inheritdoc />
    public override void _UnhandledInput(InputEvent @event)
    {
        base._UnhandledInput(@event);
    }

    /// <inheritdoc />
    public override void _Draw()
    {
        base._Draw();
    }

    /// <inheritdoc />
    public override Variant _Get(StringName property)
    {
        return property == EntitiesName ? entities : base._Get(property);
    }

    /// <inheritdoc />
    public override bool _Set(StringName property, Variant value)
    {
        if (property == EntitiesName)
        {
            List<Entity?> entitiesValue = value.AsGodotArray<Entity?>().ToList();
            FSharpMap<Eid, FSharpList<object>> internalEntities = GetEntities();

            if (internalEntities.Count == 0)
            {
                foreach (Entity entity in entitiesValue.Where(e => e is not null).Cast<Entity>())
                {
                    entity.Id = Eid.Undefined;
                }
            }

            var entitiesToDelete = new HashSet<Eid>(internalEntities.Keys);

            // Needed as godot creates items as null
            var actualArray = new Array<Entity>();

            foreach (Entity? entity in entitiesValue)
            {
                Entity actualEntity;
                if (entity?.Id.IsDefined ?? false)
                {
                    entitiesToDelete.Remove(entity.Id);
                    actualEntity = entity;
                }
                else
                {
                    actualEntity = entity ?? new Entity();
                    actualEntity.Id = AddEntity();
                    actualEntity.GameWorld = this;
                }

                actualArray.Add(actualEntity);

                SetComponents(actualEntity.Id,
                    ListModule.OfSeq(actualEntity.Components.Select(c => c.GetValue()).Where(v => v is not null)));
            }

            foreach (Eid entity in entitiesToDelete)
            {
                RemoveEntity(entity);
            }

            if (Engine.IsEditorHint())
            {
                entities = actualArray;
            }

            return true;
        }

        return base._Set(property, value);
    }

    /// <inheritdoc />
    public override Array<Dictionary> _GetPropertyList()
    {
        Array<Dictionary> properties = base._GetPropertyList() ?? new Array<Dictionary>();
        var propertyData = new Dictionary();
        propertyData["name"] = EntitiesName;
        propertyData["type"] = (int)Variant.Type.Array;
        propertyData["usage"] = (int)PropertyUsageFlags.Default;
        propertyData["hint"] = (int)PropertyHint.ResourceType;
        propertyData["hint_string"] = "Entity";
        properties.Add(propertyData);
        return properties;
    }

    [Export((PropertyHint)35, "MarginContainer")]
    public new NodePath MapUI
    {
        get => base.MapUI;
        set => base.MapUI = value;
    }

    [Export((PropertyHint)35, "Camera2D")]
    public new NodePath Camera
    {
        get => base.Camera;
        set => base.Camera = value;
    }

    [Export(PropertyHint.NodePathValidTypes, "HexMap")]
    public new NodePath Map
    {
        get => base.Map;
        set => base.Map = value;
    }

    [Export(PropertyHint.ArrayType, "PlayerData")]
    public new Array<PlayerData> Players
    {
        get => players;

        set
        {
            if (value.Where(p => p != null).GroupBy(p => p.Name).Any(g => g.Count() > 1))
            {
                GD.PrintErr("Could not set players: Duplicate names found.");
                return;
            }

            var actualPlayers = new Array<PlayerData>();
            var newPlayers = new List<Tuple<string, Player.PlayerData>>();
            foreach (PlayerData player in value)
            {
                PlayerData actualPlayerData;

                if (player is not null)
                {
                    actualPlayerData = player;
                    player.GameWorld = this;
                }
                else
                {
                    var name = "Player 0";
                    var counter = 1;
                    while (base.Players.ContainsKey(name))
                    {
                        name = $"Player {counter++}";
                    }

                    actualPlayerData = new PlayerData { Name = name, GameWorld = this };
                }

                actualPlayers.Add(actualPlayerData);
                newPlayers.Add(new Tuple<string, Player.PlayerData>(actualPlayerData.Name,
                    new Player.PlayerData(Player.ColorFromGodotColor(actualPlayerData.Color))));
            }

            SyncPlayers(newPlayers);

            if (Engine.IsEditorHint())
            {
                players = actualPlayers;
            }
        }
    }

    public void PlayerChanged()
    {
        var newPlayers = new List<Tuple<string, Player.PlayerData>>();
        foreach (PlayerData player in players)
        {
            newPlayers.Add(new Tuple<string, Player.PlayerData>(player.Name,
                new Player.PlayerData(Player.ColorFromGodotColor(player.Color))));
        }

        SyncPlayers(newPlayers);
    }

    private void SyncPlayers(IEnumerable<Tuple<string, Player.PlayerData>> argPlayers)
    {
        base.Players = new FSharpMap<string, Player.PlayerData>(argPlayers);
    }

    public List<string> GetPlayerNames()
    {
        return base.Players.Keys.ToList();
    }
}
