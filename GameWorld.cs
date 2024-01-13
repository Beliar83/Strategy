using System;
using System.Collections.Generic;
using System.Linq;
using Godot;
using Godot.Collections;
using Microsoft.FSharp.Collections;
using Strategy.FSharp;

namespace Strategy;

[Tool]
public partial class GameWorld : FSharp.GameWorld.GameWorld
{
    private static readonly StringName EntityContainerName = new("EntityContainer");

    private EntityContainer entityContainer;

    [Export(PropertyHint.ResourceType, "EntityContainer")]
    public EntityContainer EntityContainer
    {
        get => entityContainer;
        set
        {
            value.GameWorld = this;
            if (Engine.IsEditorHint())
            {
                entityContainer = value;
            }
        }
    }

    private Array<PlayerData> players = new();

    public GameWorld()
    {
        SyncPlayers(new List<Tuple<string, Player.PlayerData>>());
        entityContainer = new EntityContainer { GameWorld = this };
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
            foreach (Entity entity in entityContainer.GetEntities())
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
