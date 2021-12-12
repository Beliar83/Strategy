module Strategy.FSharp.HexMap

open Godot
open Godot.Collections
open Strategy.FSharp.Hexagon
open System.Collections.Generic


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

    member this.CellSize
        with get () = cellSize / 10f
        and set value =
            cellSize <- value * 10f
            this.UpdateCells()

    member this.Cells
        with get () = cells
        and set value =
            cells <- value
            this.UpdateCells()



    member this.SelectedCell
        with get (): Option<Hexagon> = selectedCell
        and set value =
            if not (selectedCell = value) then
                match selectedCell with
                | Some cell ->
                    if cellNodes.ContainsKey cell then
                        (GD.InstanceFromId cellNodes.[cell])
                            .EmitSignal("Deselected")
                | None -> ()

                selectedCell <- value

                match selectedCell with
                | Some cell ->
                    if cellNodes.ContainsKey cell then
                        (GD.InstanceFromId cellNodes.[cell])
                            .EmitSignal("Selected")
                | None -> ()

    member this.UpdateCells() =
        while this.GetChildCount() > 0 do
            let node = this.GetChildOrNull<Node> 0

            if not <| isNull node then
                node.QueueFree()
                this.RemoveChild node

        let hexagon =
            GD.Load("res://Hexagon.tscn") :?> PackedScene

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

let GetNeighbours (hexagon: Hexagon) =
    [| hexagon.GetNeighbour(Direction.East),
       hexagon.GetNeighbour(Direction.NorthEast),
       hexagon.GetNeighbour(Direction.NorthWest),
       hexagon.GetNeighbour(Direction.West),
       hexagon.GetNeighbour(Direction.SouthWest),
       hexagon.GetNeighbour(Direction.SouthEast) |]
