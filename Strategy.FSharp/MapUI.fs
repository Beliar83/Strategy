module Strategy.FSharp.MapUI

open System
open Godot
open Strategy.FSharp.Player

type MapUI() =
    inherit MarginContainer()

    member this.SetPlayer(name: String, player: PlayerData) =
        let playerNameLabel =
            this.GetNode(new NodePath("Top/PlayerName")) :?> Label

        playerNameLabel.Text <- name
        playerNameLabel.AddColorOverride("font_color", player.Color)

    member this.ResetPlayer() =
        let playerNameLabel =
            this.GetNode(new NodePath("Top/PlayerName")) :?> Label

        playerNameLabel.Text <- String.Empty
