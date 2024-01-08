﻿module Strategy.FSharp.MapUI

open System
open Godot
open Garnet.Composition
open Microsoft.FSharp.Core
open Strategy.FSharp.Player
open Strategy.FSharp.Systems

type MapUI() =
    inherit MarginContainer()

    let mutable playerLabel: Option<NodePath> = None

    member this.PlayerLabel
        with get () =
            match playerLabel with
            | Some path -> path
            | None -> null
        and set value =
            if isNull value then
                playerLabel <- None
            else
                playerLabel <- Some(value)

    member this.SetPlayer(name: String, player: PlayerData) =
        match playerLabel with
        | None -> GD.PrintErr("MapUI: PlayerLabel is not set")
        | Some playerLabel ->
            let playerNameLabel = this.GetNode(playerLabel) :?> Label

            playerNameLabel.Text <- name
            playerNameLabel.AddThemeColorOverride("font_color", GodotColorFromColor(player.Color))

    member this.ResetPlayer() =
        match playerLabel with
        | None -> GD.PrintErr("MapUI: PlayerLabel is not set")
        | Some playerLabel ->
            let playerNameLabel = this.GetNode(playerLabel) :?> Label

            playerNameLabel.Text <- String.Empty

    member this.ShowContextMenu (items: List<MenuItem>) (position: Vector2I) (closed: FSharp.Core.Unit -> FSharp.Core.Unit) =
        let scene = GD.Load "res://PopupMenu.tscn" :?> PackedScene
        let contextMenu = scene.Instantiate() :?> PopupMenu
        this.AddChild(contextMenu)
        contextMenu.Clear()

        items
        |> List.iteri
            (fun index item ->
                match item.ItemType with
                | IconItem iconPath ->
                    let icon = ResourceLoader.Load<Texture2D>(iconPath)
                    contextMenu.AddIconItem(icon, item.Label, index)
                | Item -> contextMenu.AddItem(item.Label, index))

        contextMenu.ResetSize()
        let idPressed =
            contextMenu.ToSignal(contextMenu, "id_pressed")

        idPressed.OnCompleted
            (fun () ->
                let result = idPressed.GetResult()
                let index = result[0].AsInt32()

                if index >= 0 then
                    let item = List.item index items
                    item.Command()
                else
                    closed ())

        let popupHide = contextMenu.ToSignal(contextMenu, "popup_hide")
        popupHide.OnCompleted(
            fun () ->                    
                let checkResult() =
                    if not <| idPressed.IsCompleted then
                        closed()
                    contextMenu.QueueFree()                    
                    
                Async.StartWithContinuations(Async.Sleep 100, checkResult, (fun _ -> checkResult()), (fun _ -> checkResult()))
                
            )
        
        contextMenu.Position <- position
        contextMenu.Popup()

module MapUISystem =
    let registerShowCellMenu (c: Container) =
        c.On<ShowCellMenu>
        <| fun event ->
            let uiNode = c.LoadResource<uint64>("UINode")

            let uiNode =
                GodotObject.InstanceFromId(uiNode) :?> MapUI

            uiNode.ShowContextMenu event.Items event.Position event.ClosedHandler

    let register (c: Container) =
        Disposable.Create [ registerShowCellMenu c ]
