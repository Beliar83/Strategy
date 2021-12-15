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

type HexMap() =
    inherit Node2D()
    let mutable cellSize = 40.0f
    let mutable cells = Array<Vector2>()
    let cellNodes = Dictionary<Hexagon, uint64>()
    let mutable selectedCell = None
    let mutable cursorCell = None

    let emitCellSignal signal cell =        
        (GD.InstanceFromId cellNodes.[cell])
            .EmitSignal signal

    let emitCellSelected = emitCellSignal "selected"
    let emitCellDeselected = emitCellSignal "deselected"
    let emitCursorEnteredCell = emitCellSignal "cursor_entered"
    let emitCursorExitedCell = emitCellSignal "cursor_exited"
    
    member this.CellSize
        with get () = cellSize
        and set value =
            cellSize <- value
            this.UpdateCells()

    member this.Cells
        with get () = cells
        and set value =
            cells <- value
            this.UpdateCells()


    member this.SelectedCell: Option<Hexagon> = selectedCell
    member this.CursorCell : Option<Hexagon> = cursorCell

    member this.SelectCell(cell: Hexagon) =
        let sameCell =
            match selectedCell with
            | Some selectedCell -> cell = selectedCell
            | None -> false    
        
        if not sameCell then
            match selectedCell with
            | Some selectedCell ->
                if cellNodes.ContainsKey(selectedCell) then
                    emitCellDeselected selectedCell
            | None -> ()
            
            
            if cellNodes.ContainsKey(cell) then
                selectedCell <- Some(cell)
                emitCellSelected cell
            else
                selectedCell <- None


    member this.UpdateCells() =
        while this.GetChildCount() > 0 do
            let node = this.GetChildOrNull<Godot.Node> 0

            if not <| isNull node then
                node.QueueFree()
                this.RemoveChild node

        selectedCell <- None
        cursorCell <- None

        let hexagon =
            GD.Load("res://Hexagon.tscn") :?> PackedScene

        cellNodes.Clear()

        for cell in this.Cells do
            let node = hexagon.Instance() :?> Node2D
            let cell = Hexagon.FromVector2 cell
            node.Position <- cell.Get2DPosition(float32 cellSize)
            node.Scale <- Vector2(cellSize, cellSize)
            node.Set("Cell", Vector3(float32 cell.Q, float32 cell.R, float32 cell.S))
            cellNodes.[cell] <- node.GetInstanceId()
            this.AddChild node
            this.Update()

    member this.GetCellAtPosition(position: Vector2) = Hexagon.At2DPosition cellSize position
    
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
            let cellsNode = c.LoadResource<uint64>("CellsNode")
            let cellsNode = GD.InstanceFromId(cellsNode) :?> HexMap
            for entity in c.Query<Eid, Hexagon>() do
                let entityId = entity.Value1
                let hexagon = entity.Value2
                let entity = c.Get entityId
                let position = hexagon.Get2DPosition cellsNode.CellSize
                entity.Add {X = position.x; Y = position.y}
                
    let registerUpdateSelected(c: Container) =
        c.On<Update>
        <| fun _ ->

            let cellsNode = c.LoadResource<uint64>("CellsNode")
            let cellsNode = GD.InstanceFromId(cellsNode) :?> HexMap

            let mousePosition =
                c.LoadResource<Vector2> "CursorPosition"

            let cell =
                cellsNode.GetCellAtPosition mousePosition
            
            c.Send <| { Cell = cell }
                
    let registerCursorEntered (c: Container) =
        c.On<CursorMoved>
        <| fun event ->
            let cellsNode = c.LoadResource<uint64>("CellsNode")
            let cellsNode = GD.InstanceFromId(cellsNode) :?> HexMap
            
            cellsNode.UpdateCursorCell event.Cell
            
    let registerSelectPressed (c: Container) =
        c.On<ButtonPressed>
        <| fun event ->
            match event.Button with
            | Select ->
                let cellsNode = c.LoadResource<uint64>("CellsNode")
                let cellsNode = GD.InstanceFromId(cellsNode) :?> HexMap
                match cellsNode.CursorCell with
                | Some cell -> cellsNode.SelectCell cell
                | None -> ()


    let register (c: Container) =
        Disposable.Create [
            registerUpdatePosition c
            registerUpdateSelected c
            registerCursorEntered c
            registerSelectPressed c
        ]
    
    
                
    
                
