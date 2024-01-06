module Strategy.FSharp.Systems

open System
open Godot
open Garnet.Composition
open Microsoft.FSharp.Core
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
    | ContextMenu of GameState
    | Selected of Hexagon * Option<Eid>
    | Moving of Eid * list<Hexagon>
    | Attacking of Eid * Eid

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

type Unit =
    { Integrity: int32
      Damage: int32
      MaxAttackRange: int32
      MinAttackRange: int32
      Armor: int32
      Mobility: int32
      RemainingRange: int32
      RemainingAttacks: int32 }

let rec getChangedState (container: Container) currentState newState =
    let getChangedState = getChangedState container

    let selectCell (cell: Hexagon) =
        container.AddResource("FieldsNeedUpdate", true)
        container.Send { SelectedCell = cell }

    let deselectCell (cell: Hexagon) =
        container.Send { DeselectedCell = cell }
        container.AddResource("FieldsNeedUpdate", true)

    let resetUnits () =
        for entity in container.Query<Eid, Unit>() do
            let unit = entity.Value2

            container
                .Get(entity.Value1)
                .Set(
                    { unit with
                          RemainingRange = unit.Mobility
                          RemainingAttacks = 1 }
                )

    let changeFromStartup () =
        match newState with
        | NewRound -> newState
        | _ -> currentState

    let changeFromWaiting () =
        match newState with
        | Startup
        | ContextMenu _
        | Waiting -> newState
        | Selected (cell, _) ->
            selectCell (cell)
            newState
        | Moving _
        | Attacking _ -> currentState
        | NewRound ->
            resetUnits ()
            newState

    let changeFromSelected (cell, eid) =
        match newState with
        | Startup ->
            deselectCell (cell)
            currentState
        | NewRound ->
            resetUnits ()
            deselectCell (cell)
            newState
        | Attacking (attacker, _) ->
            match eid with
            | None -> currentState
            | Some selected ->
                if selected = attacker then
                    newState
                else
                    currentState
        | Selected (cell, _) ->
            selectCell (cell)
            newState
        | _ ->
            deselectCell (cell)
            newState

    let changeFromNewRound () =
        match newState with
        | Waiting -> newState
        | _ -> currentState

    let changeFromMoving (currentEid, currentPath: Hexagon list) =
        match newState with
        | Waiting ->
            if (currentPath.Length <= 0) then
                newState
            else
                currentState
        | Moving (eid, path) ->
            if (eid = currentEid
                && path.Length < currentPath.Length) then
                newState
            else
                currentState
        | _ -> currentState

    let changeStateFromAttacking () =
        match newState with
        | ContextMenu _ -> newState
        | Waiting -> newState
        | Selected (cell, _) ->
            selectCell (cell)
            newState
        | _ -> currentState

    let changeFromContextMenu storedState = getChangedState (storedState)

    match currentState with
    | Startup -> changeFromStartup ()
    | Waiting -> changeFromWaiting ()
    | Selected (cell, eid) -> changeFromSelected (cell, eid)
    | NewRound -> changeFromNewRound ()
    | Moving (currentEid, currentPath) -> changeFromMoving (currentEid, currentPath)
    | Attacking _ -> changeStateFromAttacking ()
    | ContextMenu storedState -> changeFromContextMenu storedState newState

let ChangeState newState (container: Container) =
    let currentState =
        container.LoadResource<GameState> "State"

    let changedState =
        getChangedState container currentState newState

    let worldNode =
        GodotObject.InstanceFromId(container.LoadResource<uint64>("WorldNode")) :?> Node2D

    worldNode.QueueRedraw()


    container.AddResource("State", changedState)
