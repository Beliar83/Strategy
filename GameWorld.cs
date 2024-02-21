using System;
using System.Collections.Generic;
using System.Linq;
using Godot;
using Godot.Collections;
using Strategy.Components;
// using Strategy.FSharp;

namespace Strategy;

[Tool]
[GlobalClass]
public partial class GameWorld : Node2D
{
    private static readonly StringName EntityContainerName = new("EntityContainer");

    private Array<PlayerData> players = new();

    private readonly StringName playersName = new("Players");

    public GameWorld()
    {
        // SyncPlayers(new List<Tuple<string, Player.PlayerData>>());
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
    public NodePath MapUI
    {
        get;
        set;
    }

    [Export((PropertyHint)35, "Camera2D")]
    public NodePath Camera { get; set; }

    [Export(PropertyHint.NodePathValidTypes, "HexMap")]
    public NodePath Map { get; set; }

    [Export(PropertyHint.ArrayType, "PlayerData")]
    public Array<PlayerData> Players
    {
        get => players;

        set
        {
            if (value.Where(p => p != null).GroupBy(p => p.Name).Any(g => g.Count() > 1))
            {
                GD.PrintErr("Could not set players: Duplicate names found.");
                return;
            }

            players = value;
            // var actualPlayers = new Array<PlayerData>();
            // var newPlayers = new List<Tuple<string, PlayerData>>();
            // foreach (PlayerData player in value)
            // {
            //     PlayerData actualPlayerData;
            //
            //     if (player is not null)
            //     {
            //         actualPlayerData = player;
            //     }
            //     else
            //     {
            //         var name = "Player 0";
            //         var counter = 1;
            //         while (Players.Any(p => p.Name == name))
            //         {
            //             name = $"Player {counter++}";
            //         }
            //
            //         actualPlayerData = new PlayerData { Name = name};
            //     }
            //
            //     actualPlayers.Add(actualPlayerData);
            // }
            // //
            // // SyncPlayers(newPlayers);
            // //
            // // if (Engine.IsEditorHint())
            // // {
            // //     players = actualPlayers;
            // // }
        }
    }

    public void PlayerChanged()
    {
        // var newPlayers = new List<Tuple<string, Player.PlayerData>>();
        // foreach (PlayerData player in players)
        // {
        //     newPlayers.Add(new Tuple<string, Player.PlayerData>(player.Name,
        //         new Player.PlayerData(Player.ColorFromGodotColor(player.Color))));
        // }
        //
        // SyncPlayers(newPlayers);
    }

    // private void SyncPlayers(IEnumerable<Tuple<string, Player.PlayerData>> argPlayers)
    // {
    //     // base.Players = new FSharpMap<string, Player.PlayerData>(argPlayers);
    // }
    //
    // public List<string> GetPlayerNames()
    // {
    //     // return base.Players.Keys.ToList();
    // }

}
