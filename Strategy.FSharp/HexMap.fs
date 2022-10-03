module Strategy.FSharp.HexMap

open Garnet.Composition.Join
open Godot
open Microsoft.FSharp.Core
open Strategy.FSharp.Hexagon
open Garnet.Composition
open Strategy.FSharp.Input
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
    let mutable cursorCell = None

    member this.CursorCell: Option<Hexagon> = cursorCell

    member this.UpdateCells(container: Container) =
        while this.GetChildCount() > 0 do
            let node = this.GetChildOrNull<Node> 0

            if not <| isNull node then
                node.QueueFree()
                this.RemoveChild node

        cursorCell <- None

        let hexagon =
            GD.Load("res://Hexagon.tscn") :?> PackedScene
        

        let cells = container.LoadResource<Hexagon[]>("Cells")
        
        let cell_nodes =
            cells
            |> Array.map (fun cell ->
                let node = hexagon.Instantiate() :?> Node2D
                let cell = Hexagon.FromVector2 cell.AsVector2
                node.Position <- cell.Get2DPosition
                node.Set("Cell", Vector3(float32 cell.Q, float32 cell.R, float32 cell.S))
                this.AddChild node
                this.QueueRedraw()
                (cell, node.GetInstanceId()) )
        
        let cell_nodes = Map.ofArray cell_nodes
        
        container.AddResource("CellNodes", cell_nodes)

    member this.GetCellAtPosition(position: Vector2) = Hexagon.At2DPosition position


let GetNeighbours (hexagon: Hexagon) =
    [| hexagon.GetNeighbour(Direction.East),
       hexagon.GetNeighbour(Direction.NorthEast),
       hexagon.GetNeighbour(Direction.NorthWest),
       hexagon.GetNeighbour(Direction.West),
       hexagon.GetNeighbour(Direction.SouthWest),
       hexagon.GetNeighbour(Direction.SouthEast) |]


[<Struct>]
type CellsUpdated = struct end

[<Struct>]
type UpdateSelectedCell = {Cell: Hexagon}

let GetCellAtPosition(position: Vector2) = Hexagon.At2DPosition position

let emitCellSignal signal cell (cellNodes : Map<Hexagon, uint64>) =
    let node = (GD.InstanceFromId cellNodes[cell])
    node.EmitSignal signal

let emitCellSelected =
    emitCellSignal (new StringName "selected")

let emitCellDeselected =
    emitCellSignal (new StringName "deselected")

let emitCursorEnteredCell =
    emitCellSignal (new StringName "cursor_entered")

let emitCursorExitedCell =
    emitCellSignal (new StringName "cursor_exited")

let UpdateCursorCell(cell: Hexagon) (c: Container) =
        let cursorCell = c.LoadResource<Option<Hexagon>>("CursorCell")
        let sameCell =
            match cursorCell with
            | Some currentCell -> cell = currentCell
            | None -> false

        let cellNodes = c.LoadResource<Map<Hexagon, uint64>>("CellNodes")

        if not sameCell then
            match cursorCell with
            | Some currentCell ->
                if cellNodes.ContainsKey(currentCell) then
                    emitCursorExitedCell currentCell cellNodes |> ignore
            | None -> ()

            if cellNodes.ContainsKey(cell) then
                c.AddResource("CursorCell", Some(cell))
                emitCursorEnteredCell cell cellNodes |> ignore
            else
                c.AddResource("CursorCell", Option<Hexagon>.None)

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

            let mousePosition = c.LoadResource<Vector2> "CursorPosition"

            let cell = GetCellAtPosition mousePosition

            c.Send <| { CursorCell = cell }

    let registerCursorEntered (c: Container) =
        c.On<CursorMoved>
        <| fun event ->
            UpdateCursorCell event.CursorCell c

    let registerSelectPressed (c: Container) =

        let handleSelect () =
            let state = c.LoadResource<GameState>("State")

            let getEntitiesAtCell (cell: Hexagon) =
                c.Query<Eid, Hexagon>()
                |> Seq.filter (fun x -> x.Value2 = cell)
                |> Seq.map (fun x -> c.Get x.Value1)
                |> Seq.toArray

            let showContextMenuForCell (cell: Hexagon) =
                let items = [||]

                let entities = getEntitiesAtCell cell

                let selectCell (cell: Hexagon) =
                    c.Send { SelectedCell = cell }
                let entity_command entity_id =
                    ChangeState (GameState.Selected(cell, Some(entity_id))) c

                    selectCell cell

                let getItemForUnit (entity: Entity) (_unit: Unit) =
                    { Label = "Unit"
                      Command = (fun () -> entity_command entity.Id)
                      ItemType = ItemType.IconItem("res://assets/units/tank.png") }

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
                    ChangeState GameState.NewRound c

                let close () =
                    c.AddResource("CursorPosition", camera.GetLocalMousePosition())

                let items =
                    Array.append
                        items
                        [| { Label = "End Turn"
                             Command = end_turn
                             ItemType = ItemType.IconItem "res://assets/icons/simpleBlock.png" } |]

                let items =
                    Array.append
                        items
                        [| { Label = "Close"
                             Command = close
                             ItemType = ItemType.Item } |]

                c.Run {Items = Array.toList items; Position = Vector2i.op_Explicit position; ClosedHandler = close}

            match state with
            | GameState.Startup -> ()
            | GameState.NewRound -> ()
            | GameState.Selected _
            | GameState.Waiting ->
                let cursorCell = c.LoadResource<Option<Hexagon>>("CursorCell")

                match cursorCell with
                | Some cursorCell -> showContextMenuForCell cursorCell
                | None -> ()
            // TODO: Attacking, Moving

        let handleCancel () =
            match c.LoadResource<GameState>("State") with
            | Selected (cell, _) ->
                c.Send { DeselectedCell = cell }
                ChangeState Waiting c
            | _ -> ()

        c.On<ButtonPressed>
        <| fun event ->
            match event.Button with
            | Select -> handleSelect ()
            | Cancel -> handleCancel ()
    
    let registerCellSelected (container: Container) =                
        container.On<SelectCell>
        <| fun event ->
            let cellNodes = container.LoadResource<Map<Hexagon, uint64>>("CellNodes")
            if cellNodes.ContainsKey(event.SelectedCell) then
                emitCellSelected event.SelectedCell cellNodes |> ignore
    
    let registerCellDeselected (container: Container) =
        container.On<DeselectCell>
        <| fun event ->
            let cellNodes = container.LoadResource<Map<Hexagon, uint64>>("CellNodes")
            if cellNodes.ContainsKey(event.DeselectedCell) then
                emitCellDeselected event.DeselectedCell cellNodes |> ignore

    let registerCellsUpdated (container :Container) =
        container.On<CellsUpdated>
        <| fun _ ->
            let hexMap = container.LoadResource<uint64>("CellsNode")
            let hexMap = GD.InstanceFromId(hexMap) :?> HexMap
            hexMap.UpdateCells container
    
    let register (c: Container) =
        Disposable.Create [ registerUpdatePosition c
                            registerUpdateSelected c
                            registerCursorEntered c
                            registerSelectPressed c
                            registerCellSelected c
                            registerCellDeselected c
                            registerCellsUpdated c]