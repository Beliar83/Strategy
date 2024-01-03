module Strategy.FSharp.Systems

open System
open Godot
open Garnet.Composition
open Strategy.FSharp.Hexagon

[<Struct>]
type PhysicsUpdate = { PhysicsDelta: float }

[<Struct>]
type FrameUpdate = { FrameDelta : float }

[<Struct>]
type Position = { X: float32; Y: float32 }

type GameState =
    | Startup
    | NewRound
    | Waiting
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

let ChangeState new_state (container: Container) =
    let state =
        container.LoadResource<GameState> "State"

    let changed_state =
        match state with
        | Startup ->
            match new_state with
            | NewRound -> new_state
            | _ -> state
        | Waiting ->
            match new_state with
            | Startup
            | NewRound
            | Waiting -> new_state
            | Selected _ -> new_state
            | Moving _ -> state
        | Selected (cell, _) ->
            match new_state with
            | Waiting ->
                container.Send { DeselectedCell = cell }
                new_state
            | Moving _ ->
                container.Send { DeselectedCell = cell }
                new_state
            | Selected _ ->
                container.Send { DeselectedCell = cell }
                new_state
            | _ -> state
        | NewRound ->
            match new_state with
            | Waiting -> new_state
            | _ -> state
        | Moving(currentEid, currentPath) ->
            match new_state with
            | Waiting -> if (currentPath.Length <= 0) then new_state else state
            | Moving(eid, path) -> if (eid = currentEid && path.Length < currentPath.Length) then new_state else state
            | _ -> state
        

    container.AddResource("State", changed_state)
