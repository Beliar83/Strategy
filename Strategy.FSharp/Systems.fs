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

type Unit =
    { Integrity: int32
      Damage: int32
      MaxAttackRange: int32
      MinAttackRange: int32
      Armor: int32
      Mobility: int32
      RemainingRange: int32
      RemainingAttacks: int32 }

let ChangeState newState (container: Container) =
    let state =
        container.LoadResource<GameState> "State"

    let selectCell(cell: Hexagon) = 
        container.AddResource("FieldsNeedUpdate", true)
        container.Send { SelectedCell = cell }
    
    let deselectCell(cell: Hexagon) =
        container.Send { DeselectedCell = cell }
        container.AddResource("FieldsNeedUpdate", true)        
    
    let resetUnits() =
        for entity in container.Query<Eid, Unit>() do
            let unit = entity.Value2
            container.Get(entity.Value1).Set( { unit with RemainingRange = unit.Mobility; RemainingAttacks = 1 })
    
    let changedState =
        match state with
        | Startup ->
            match newState with
            | NewRound -> newState
            | _ -> state
        | Waiting ->
            match newState with
            | Startup
            | ContextMenu
            | Waiting -> newState
            | Selected (cell, _) ->
                    selectCell(cell)
                    newState                
            | Moving _ -> state
            | NewRound ->
                resetUnits()
                newState
        | Selected (cell, _) ->
            match newState with
            | Startup ->
                deselectCell(cell)
                state
            | NewRound ->
                resetUnits()
                deselectCell(cell)
                newState
            | Selected _ -> state
            | _ ->
                deselectCell(cell)
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
                selectCell(cell)
                newState
            | NewRound ->
                resetUnits()
                newState
            | _ -> newState
            
    let worldNode = GodotObject.InstanceFromId(container.LoadResource<uint64>("WorldNode")) :?> Node2D            
    worldNode.QueueRedraw()


    container.AddResource("State", changedState)
