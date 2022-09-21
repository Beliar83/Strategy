module Strategy.FSharp.MapUI

open System
open Godot
open Godot.Collections
open Strategy.FSharp.Player

type MapUI() =
    inherit MarginContainer()

    let mutable player_label: Option<NodePath> = None
    let mutable radial_menu: Option<NodePath> = None

    member this.PlayerLabel
        with get () =
            match player_label with
            | Some path -> path
            | None -> null
        and set value =
            if isNull value then
                player_label <- None
            else
                player_label <- Some(value)

    member this.RadialMenu
        with get () =
            match radial_menu with
            | Some path -> path
            | None -> null
        and set value =
            if isNull value then
                radial_menu <- None
            else
                radial_menu <- Some(value)


    member this.SetPlayer(name: String, player: PlayerData) =
        match player_label with
        | None -> GD.PrintErr("MapUI: PlayerLabel is not set")
        | Some player_label ->
            let playerNameLabel = this.GetNode(player_label) :?> Label

            playerNameLabel.Text <- name
            playerNameLabel.AddThemeColorOverride("font_color", player.Color)

    member this.ResetPlayer() =
        match player_label with
        | None -> GD.PrintErr("MapUI: PlayerLabel is not set")
        | Some player_label ->
            let playerNameLabel = this.GetNode(player_label) :?> Label

            playerNameLabel.Text <- String.Empty

    member this.ShowRadialMenu (items: Dictionary []) (position: Vector2) =
        match radial_menu with
        | None ->
            GD.PrintErr("MapUI: RadialMenu is not set")
            async { return None }
        | Some radial_menu ->
            let items =
                let array = new Array<Dictionary>()

                for item in items do
                    array.Add(item)

                array

            let radial_menu = this.GetNode(radial_menu) :?> Popup
            radial_menu.Set("menu_items", items)
            radial_menu.Call("open_menu", position) |> ignore

            async {
                let item_selected =
                    radial_menu.ToSignal(radial_menu, "item_selected")

                let cancelled =
                    radial_menu.ToSignal(radial_menu, "cancelled")

                while not item_selected.IsCompleted
                      && not cancelled.IsCompleted do
                    Async.Sleep(TimeSpan.FromMilliseconds(1000.0))
                    |> ignore

                if item_selected.IsCompleted then
                    return Some(item_selected.GetResult())
                else
                    return None
            }
            |> Async.StartChild
            |> Async.RunSynchronously
