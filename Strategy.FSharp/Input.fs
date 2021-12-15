module Strategy.FSharp.Input
open Strategy.FSharp.Hexagon

type Button =
    | Select

[<Struct>]
type CursorMoved = { Cell: Hexagon }

[<Struct>]
type ButtonPressed = { Button : Button }
