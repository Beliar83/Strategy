module Strategy.FSharp.Systems

open System
open Godot
open Garnet.Composition
open Strategy.FSharp.Hexagon

[<Struct>]
type Update = { UpdateTime: float }

[<Struct>]
type Position = { X: float32; Y: float32 }

type GameState =
    | Startup
    | NewRound
    | Waiting
    | Selected of Hexagon * Option<Eid>

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
      Position: Vector2i
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
            | _ -> new_state
        | Selected (cell, _) ->
            container.Send { DeselectedCell = cell }
            new_state
        | NewRound -> new_state

    container.AddResource("State", changed_state)
