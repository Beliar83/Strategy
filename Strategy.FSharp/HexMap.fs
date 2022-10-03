module Strategy.FSharp.HexMap

open Garnet.Composition.Join
open Godot
open Godot.Collections
open Strategy.FSharp.Hexagon
open Garnet.Composition
open Strategy.FSharp.Input
open Strategy.FSharp.MapUI
open Strategy.FSharp.Systems
open Strategy.FSharp.Unit

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

type HexMap() =
    inherit Node2D()
    let mutable cells = Array<Vector2>()

    let cellNodes =
        System.Collections.Generic.Dictionary<Hexagon, uint64>()

    let mutable cursorCell = None

    let emitCellSignal signal cell =
        (GD.InstanceFromId cellNodes.[cell])
            .EmitSignal signal

    let emitCellSelected =
        emitCellSignal (new StringName "selected")

    let emitCellDeselected =
        emitCellSignal (new StringName "deselected")

    let emitCursorEnteredCell =
        emitCellSignal (new StringName "cursor_entered")

    let emitCursorExitedCell =
        emitCellSignal (new StringName "cursor_exited")

    member this.Cells
        with get () = cells
        and set value =
            cells <- value
            this.UpdateCells()

    member this.CursorCell: Option<Hexagon> = cursorCell

    member this.SelectCell(cell: Hexagon) =
        if cellNodes.ContainsKey(cell) then
            emitCellSelected cell |> ignore

    member this.DeselectCell(cell) =
        if cellNodes.ContainsKey(cell) then
            emitCellDeselected cell |> ignore


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
            let node = hexagon.Instantiate() :?> Node2D
            let cell = Hexagon.FromVector2 cell
            node.Position <- cell.Get2DPosition
            node.Set("Cell", Vector3(float32 cell.Q, float32 cell.R, float32 cell.S))
            cellNodes.[cell] <- node.GetInstanceId()
            this.AddChild node
            this.QueueRedraw()

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
                    emitCursorExitedCell currentCell |> ignore
            | None -> ()

            if cellNodes.ContainsKey(cell) then
                cursorCell <- Some(cell)
                emitCursorEnteredCell cell |> ignore
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

        let handleSelect () =
            let state = c.LoadResource<GameState>("State")

            let getEntitiesAtCell (cell: Hexagon) =
                c.Query<Eid, Hexagon>()
                |> Seq.filter (fun x -> x.Value2 = cell)
                |> Seq.map (fun x -> c.Get x.Value1)
                |> Seq.toArray


            let showContextMenuForCell (cell: Hexagon) =
                // Add actual items
                let uiNode = c.LoadResource<uint64>("UINode")
                let uiNode = GD.InstanceFromId(uiNode) :?> MapUI

                let items = [||]

                let entities = getEntitiesAtCell cell

                let selectCell (hexMap: HexMap) (cell: Hexagon) =
                    hexMap.SelectCell cell
                    c.Send(UpdateSelection())

                let entity_command entity_id =
                    let cellsNode = c.LoadResource<uint64>("CellsNode")
                    let cellsNode = GD.InstanceFromId(cellsNode) :?> HexMap
                    c.AddResource("State", GameState.Selected(cell, Some(entity_id)))

                    match state with
                    | Selected (cell, _) -> cellsNode.DeselectCell cell
                    | _ -> ()

                    selectCell cellsNode cell

                let getItemForUnit (entity: Entity) (unit: Unit) =
                    { label = "Unit"
                      command = (fun () -> entity_command entity.Id)
                      item_type = ItemType.IconItem("res://assets/units/tank.png") }

                let items =
                    entities
                    |> Array.filter (fun entity -> entity.Has<Unit>())
                    |> Array.map (fun entity -> getItemForUnit entity (entity.Get<Unit>()))
                    |> Array.append items


                let position = cell.Get2DPosition

                let camera = c.LoadResource<uint64> "Camera"
                let camera = GD.InstanceFromId camera :?> Camera2D
                let rect = camera.GetViewportRect()
                let half_size = rect.Size / 2f
                let position = position + half_size

                let end_turn () =
                    // TODO: Actually end the turn
                    c.AddResource("State", state)
                    c.AddResource("CursorPosition", camera.GetLocalMousePosition())

                let close () =
                    c.AddResource("State", state)
                    c.AddResource("CursorPosition", camera.GetLocalMousePosition())

                let items =
                    Array.append
                        items
                        [| { label = "End Turn"
                             command = end_turn
                             item_type = ItemType.IconItem "res://assets/icons/simpleBlock.png" } |]

                let items =
                    Array.append
                        items
                        [| { label = "Close"
                             command = close
                             item_type = ItemType.Item } |]

                c.AddResource("State", GameState.ContextMenu)

                uiNode.ShowRadialMenu(Array.toList items) (Vector2i.op_Explicit (position)) close

            match state with
            | GameState.Startup -> ()
            | GameState.NewRound -> ()
            | GameState.Selected _
            | GameState.Waiting ->
                let cellsNode = c.LoadResource<uint64>("CellsNode")
                let cellsNode = GD.InstanceFromId(cellsNode) :?> HexMap

                match cellsNode.CursorCell with
                | Some cursorCell -> showContextMenuForCell cursorCell
                | None -> ()
            // TODO: Attacking, Moving
            | _ -> ()

        let handleCancel () =
            match c.LoadResource<GameState>("State") with
            | Selected (cell, _) ->
                let cellsNode = c.LoadResource<uint64>("CellsNode")
                let cellsNode = GD.InstanceFromId(cellsNode) :?> HexMap
                cellsNode.DeselectCell cell
                c.AddResource("State", Waiting)
                c.Send(UpdateSelection())
            | _ -> ()

        c.On<ButtonPressed>
        <| fun event ->
            match event.Button with
            | Select -> handleSelect ()
            | Cancel -> handleCancel ()

    let register (c: Container) =
        Disposable.Create [ registerUpdatePosition c
                            registerUpdateSelected c
                            registerCursorEntered c
                            registerSelectPressed c ]
