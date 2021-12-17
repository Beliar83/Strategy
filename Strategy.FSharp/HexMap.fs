module Strategy.FSharp.HexMap

open Godot
open Godot.Collections
open Strategy.FSharp.Hexagon
open System.Collections.Generic
open Garnet.Composition
open Strategy.FSharp.Input
open Strategy.FSharp.Systems

let PolygonWidth cellSize = sqrt 3f * cellSize

let PolygonHeight cellSize = 2f * cellSize

let Half value = value / 2f
let Quarter value = value / 4f

let HexagonPoints cellSize =
    let width = PolygonWidth cellSize
    let height = PolygonHeight cellSize
    let halfHeight = Half height
    let quarterHeight = Quarter height
    let halfWidth = Half width

    [| Vector2(-halfWidth, -quarterHeight)
       Vector2(0f, -halfHeight)
       Vector2(halfWidth, -quarterHeight)
       Vector2(halfWidth, quarterHeight)
       Vector2(0f, halfHeight)
       Vector2(-halfWidth, quarterHeight)
       Vector2(-halfWidth, -quarterHeight) |]

[<Struct>]
type CellSelected = { SelectedCell: Option<Hexagon> }

type HexMap() =
    inherit Node2D()
    let mutable cells = Array<Vector2>()
    let cellNodes = Dictionary<Hexagon, uint64>()
    let mutable cursorCell = None

    let emitCellSignal signal cell =
        (GD.InstanceFromId cellNodes.[cell])
            .EmitSignal signal

    let emitCellSelected = emitCellSignal "selected"
    let emitCellDeselected = emitCellSignal "deselected"
    let emitCursorEnteredCell = emitCellSignal "cursor_entered"
    let emitCursorExitedCell = emitCellSignal "cursor_exited"

    member this.Cells
        with get () = cells
        and set value =
            cells <- value
            this.UpdateCells()

    member this.CursorCell: Option<Hexagon> = cursorCell

    member this.SelectCell (state: GameState) (cell: Hexagon) =
        let selectNew () =
            if cellNodes.ContainsKey(cell) then
                emitCellSelected cell
                GameState.Selected(cell, None)
            else
                GameState.Waiting

        match state with
        | Selected (selectedCell, _) ->
            if not <| (cell = selectedCell) then
                this.DeselectCell(state) |> ignore
                selectNew ()
            else
                state
        | Waiting -> selectNew ()
        | _ -> state



    member this.DeselectCell(state: GameState) =
        match state with
        | Selected (selectedCell, _) ->
            if cellNodes.ContainsKey(selectedCell) then
                emitCellDeselected selectedCell

            GameState.Waiting
        | _ -> state

    member this.UpdateCells() =
        while this.GetChildCount() > 0 do
            let node = this.GetChildOrNull<Node> 0

            if not <| isNull node then
                node.QueueFree()
                this.RemoveChild node

        cursorCell <- None

        let hexagon =
            GD.Load("res://Hexagon.tscn") :?> PackedScene

        cellNodes.Clear()

        for cell in this.Cells do
            let node = hexagon.Instance() :?> Node2D
            let cell = Hexagon.FromVector2 cell
            node.Position <- cell.Get2DPosition
            node.Set("Cell", Vector3(float32 cell.Q, float32 cell.R, float32 cell.S))
            cellNodes.[cell] <- node.GetInstanceId()
            this.AddChild node
            this.Update()

    member this.GetCellAtPosition(position: Vector2) = Hexagon.At2DPosition position

    member this.UpdateCursorCell(cell: Hexagon) =
        let sameCell =
            match cursorCell with
            | Some currentCell -> cell = currentCell
            | None -> false

        if not sameCell then
            match cursorCell with
            | Some currentCell ->
                if cellNodes.ContainsKey(currentCell) then
                    emitCursorExitedCell currentCell
            | None -> ()

            if cellNodes.ContainsKey(cell) then
                cursorCell <- Some(cell)
                emitCursorEnteredCell cell
            else
                cursorCell <- None

let GetNeighbours (hexagon: Hexagon) =
    [| hexagon.GetNeighbour(Direction.East),
       hexagon.GetNeighbour(Direction.NorthEast),
       hexagon.GetNeighbour(Direction.NorthWest),
       hexagon.GetNeighbour(Direction.West),
       hexagon.GetNeighbour(Direction.SouthWest),
       hexagon.GetNeighbour(Direction.SouthEast) |]

module HexMapSystem =
    let registerUpdatePosition (c: Container) =
        c.On<Update>
        <| fun _ ->
            for entity in c.Query<Eid, Hexagon>() do
                let entityId = entity.Value1
                let hexagon = entity.Value2
                let entity = c.Get entityId
                let position = hexagon.Get2DPosition
                entity.Add { X = position.x; Y = position.y }

    let registerUpdateSelected (c: Container) =
        c.On<Update>
        <| fun _ ->

            let cellsNode = c.LoadResource<uint64>("CellsNode")
            let cellsNode = GD.InstanceFromId(cellsNode) :?> HexMap

            let mousePosition = c.LoadResource<Vector2> "CursorPosition"

            let cell =
                cellsNode.GetCellAtPosition mousePosition

            c.Send <| { CursorCell = cell }

    let registerCursorEntered (c: Container) =
        c.On<CursorMoved>
        <| fun event ->
            let cellsNode = c.LoadResource<uint64>("CellsNode")
            let cellsNode = GD.InstanceFromId(cellsNode) :?> HexMap

            cellsNode.UpdateCursorCell event.CursorCell

    let registerSelectPressed (c: Container) =
        c.On<ButtonPressed>
        <| fun event ->
            let state = c.LoadResource<GameState>("State")

            let cellsNode = c.LoadResource<uint64>("CellsNode")
            let cellsNode = GD.InstanceFromId(cellsNode) :?> HexMap

            let selectCursorCell () =
                match cellsNode.CursorCell with
                | Some cell -> cellsNode.SelectCell state cell
                | None -> state

            let deselectCell () = cellsNode.DeselectCell state

            let newState =
                match state with
                | Waiting ->
                    match event.Button with
                    | Select -> selectCursorCell ()
                    | _ -> state
                | Selected _ ->
                    match event.Button with
                    | Select -> selectCursorCell ()
                    | Cancel -> deselectCell ()
                // TODO: Attacking, Moving
                | _ -> state

            match newState with
            | Waiting -> c.Send { SelectedCell = None }
            | Selected (cell, _) -> c.Send { SelectedCell = Some(cell) }
            | _ -> ()

            c.AddResource("State", newState)


    let register (c: Container) =
        Disposable.Create [ registerUpdatePosition c
                            registerUpdateSelected c
                            registerCursorEntered c
                            registerSelectPressed c ]
