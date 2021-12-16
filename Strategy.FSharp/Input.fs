module Strategy.FSharp.Input

open Strategy.FSharp.Hexagon

type Button =
    | Select
    | Cancel

[<Struct>]
type CursorMoved = { CursorCell: Hexagon }

[<Struct>]
type ButtonPressed = { Button: Button }
