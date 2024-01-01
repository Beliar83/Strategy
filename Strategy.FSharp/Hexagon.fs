module Strategy.FSharp.Hexagon

open System
open Godot


let CELL_SIZE = 40.0f

let CubeRound x y z =
    let mutable rx = round x
    let mutable ry = round y
    let mutable rz = round z

    let xDiff = abs <| rx - x
    let yDiff = abs <| ry - y
    let zDiff = abs <| rz - z

    if xDiff > yDiff && xDiff > zDiff then
        rx <- -ry - rz
    else if yDiff > zDiff then
        ry <- -rx - rz
    else
        rz <- -rx - ry

    (int32 rx, int32 ry, int32 rz)

let CalculateAxis axis1 axis2 = -axis1 - axis2

type Direction =
    | East = 0
    | NorthEast = 1
    | NorthWest = 2
    | West = 3
    | SouthWest = 4
    | SouthEast = 5

type Hexagon =
    { Q: int32
      R: int32
      S: int32 }

    static member Zero = { Q = 0; R = 0; S = 0 }

    static member NewAxial q r =
        // https://www.redblobgames.com/grids/hexagons/#conversions-axial
        { Q = q; R = r; S = CalculateAxis q r }

    static member NewCube q r s = { Q = q; R = r; S = s }

    static member At2DPosition(pos: Vector2) =
        let q =
            ((sqrt <| 3f) / 3f * pos.X - 1f / 3f * pos.Y)
            / CELL_SIZE

        let r = (2f / 3f * pos.Y) / CELL_SIZE
        let s = -q - r
        let q, r, s = CubeRound q r s
        Hexagon.NewCube q r s

    member this.Get2DPosition =
        let x =
            CELL_SIZE
            * (sqrt 3f * float32 this.Q
               + sqrt 3f / 2f * float32 this.R)

        let y = CELL_SIZE * (3f / 2f * float32 this.R)

        Vector2(x, y)

    // Create from Vector2 representation for easy passing to and from Godot
    static member FromVector2(vector: Vector2) =
        let q = int32 vector.X
        let r = int32 vector.Y
        Hexagon.NewAxial q r

    // Represent as Vector2 for easy passing to and from Godot
    member this.AsVector2 =
        let x = float32 this.Q
        let y = float32 this.R
        Vector2(x, y)

    member self.MoveQ length =
        Hexagon.NewCube(self.Q + length) self.R (self.S - length)

    member self.MoveR length =
        Hexagon.NewCube self.Q (self.R + length) (self.S - length)

    member self.MoveS length =
        Hexagon.NewCube(self.Q - length) (self.R + length) self.S

    member self.DistanceTo(other: Hexagon) =
        // https://www.redblobgames.com/grids/hexagons/#distances-cube
        ((abs <| (self.Q - other.Q))
         + (abs <| (self.R - other.R))
         + (abs <| (self.S - other.S)))
        / 2

    member self.IsNeighbour other = self.DistanceTo other = 1

    member self.GetNeighbour direction =
        match direction with
        | Direction.East -> Hexagon.NewCube(self.Q + 1) self.R (self.S - 1)
        | Direction.NorthEast -> Hexagon.NewCube(self.Q + 1) (self.R - 1) self.S
        | Direction.NorthWest -> Hexagon.NewCube self.Q (self.R - 1) (self.S + 1)
        | Direction.West -> Hexagon.NewCube(self.Q - 1) self.R (self.S + 1)
        | Direction.SouthWest -> Hexagon.NewCube(self.Q - 1) (self.R + 1) self.S
        | Direction.SouthEast -> Hexagon.NewCube self.Q (self.R + 1) (self.S - 1)
        | _ -> raise (ArgumentOutOfRangeException("direction"))

