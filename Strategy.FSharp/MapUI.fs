module Strategy.FSharp.MapUI

open System
open Garnet.Composition
open Godot
open Microsoft.FSharp.Core
open Strategy.FSharp.Player

type ItemType =
    | Entity of Eid
    | Command of String

type MenuItem =
    | Item of label: String * item_type: ItemType
    | IconItem of icon_path: string * label: string * item_type: ItemType        

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

    member this.ShowRadialMenu (items: List<MenuItem>) (position: Vector2i) (selected: ItemType -> Unit) (closed: Unit -> Unit) =
        match radial_menu with
        | None ->
            GD.PrintErr("MapUI: RadialMenu is not set")
        | Some radial_menu ->
            
            let radial_menu = this.GetNode(radial_menu) :?> PopupMenu
            radial_menu.Clear()
            
            
            items
            |> List.iteri (fun index item ->
                match item with
                | IconItem(iconPath, label, _) ->                
                    let icon = ResourceLoader.Load<Texture2D>(iconPath)
                    radial_menu.AddIconItem(icon, label, index)
                | Item(label, _) ->
                    radial_menu.AddItem(label, index)
                )
            
            radial_menu.ResetSize()
            
            radial_menu.ToSignal(radial_menu, "popup_hide").OnCompleted(fun () -> 
                let index = radial_menu.GetFocusedItem()
                if index >= 0 then
                    match items[index] with
                    | IconItem(_, _, itemType) -> selected itemType
                    | Item(_, itemType) -> selected itemType
                else closed()
                    
            )
            
            radial_menu.Position <- position
            radial_menu.Popup()
