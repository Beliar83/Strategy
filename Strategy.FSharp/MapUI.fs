module Strategy.FSharp.MapUI

open System
open Godot
open Garnet.Composition
open Microsoft.FSharp.Core
open Strategy.FSharp.Player
open Strategy.FSharp.Systems

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

    member this.ShowRadialMenu (items: List<MenuItem>) (position: Vector2I) (closed: Unit -> Unit) =
        match radial_menu with
        | None -> GD.PrintErr("MapUI: RadialMenu is not set")
        | Some radial_menu ->

            let radial_menu = this.GetNode(radial_menu) :?> PopupMenu
            radial_menu.Clear()

            items
            |> List.iteri
                (fun index item ->
                    match item.ItemType with
                    | IconItem iconPath ->
                        let icon = ResourceLoader.Load<Texture2D>(iconPath)
                        radial_menu.AddIconItem(icon, item.Label, index)
                    | Item -> radial_menu.AddItem(item.Label, index))

            radial_menu.ResetSize()

            let event = radial_menu.ToSignal(radial_menu, "id_pressed");
            event.OnCompleted(fun () ->
                let result = event.GetResult()
                let index = result[0].AsInt32()                
                if index >= 0 then
                    let item = List.item index items
                    item.Command()
                else
                    closed ())

            radial_menu.Position <- position
            radial_menu.Popup()

module MapUISystem =
    let registerShowCellMenu (c: Container) =
        c.On<ShowCellMenu>
        <| fun event ->
            let uiNode = c.LoadResource<uint64>("UINode")
            let uiNode = GodotObject.InstanceFromId(uiNode) :?> MapUI
            uiNode.ShowRadialMenu event.Items event.Position event.ClosedHandler

    let register (c: Container) =
        Disposable.Create [ registerShowCellMenu c ]
