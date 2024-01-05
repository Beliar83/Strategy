module Strategy.FSharp.Systems

open System
open Godot
open Garnet.Composition
open Strategy.FSharp.Hexagon

[<Struct>]
type PhysicsUpdate = { PhysicsDelta: float }

[<Struct>]
type FrameUpdate = { FrameDelta: float }

[<Struct>]
type Position = { X: float32; Y: float32 }

type UnitPosition =
    { Position: Hexagon
      BodyRotation: float32
      WeaponRotation: float32 }

type GameState =
    | Startup
    | NewRound
    | Waiting
    | ContextMenu
    | Selected of Hexagon * Option<Eid>
    | Moving of Eid * list<Hexagon>

[<Struct>]
type SelectCell = { SelectedCell: Hexagon }

[<Struct>]
type DeselectCell = { DeselectedCell: Hexagon }

type ItemType =
    | Item
    | IconItem of IconPath: string

type MenuItem =
    { Label: String
      Command: Unit -> Unit
      ItemType: ItemType }

[<Struct>]
type ShowCellMenu =
    { Items: List<MenuItem>
      Position: Vector2I
      ClosedHandler: Unit -> Unit }

let ChangeState newState (container: Container) =
    let state =
        container.LoadResource<GameState> "State"

    let changedState =
        match state with
        | Startup ->
            match newState with
            | NewRound -> newState
            | _ -> state
        | Waiting ->
            match newState with
            | Startup
            | NewRound
            | ContextMenu
            | Waiting -> newState
            | Selected (cell, _) ->
                    container.AddResource("FieldsNeedUpdate", true)
                    container.Send { SelectedCell = cell }
                    newState                
            | Moving _ -> state
        | Selected (cell, _) ->
            match newState with
            | Startup -> state
            | _ ->
                container.Send { DeselectedCell = cell }
                container.AddResource("FieldsNeedUpdate", true)
                newState
        | NewRound ->
            match newState with
            | Waiting -> newState
            | _ -> state
        | Moving (currentEid, currentPath) ->
            match newState with
            | Waiting ->
                if (currentPath.Length <= 0) then
                    newState
                else
                    state
            | Moving (eid, path) ->
                if (eid = currentEid
                    && path.Length < currentPath.Length) then
                    newState
                else
                    state
            | _ -> state
        | ContextMenu ->
            match newState with
            | Selected (cell, _) ->
                container.AddResource("FieldsNeedUpdate", true)
                container.Send { SelectedCell = cell }
                newState
            | _ -> newState
            
    let worldNode = GodotObject.InstanceFromId(container.LoadResource<uint64>("WorldNode")) :?> Node2D            
    worldNode.QueueRedraw()


    container.AddResource("State", changedState)
