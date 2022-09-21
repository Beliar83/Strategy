module Strategy.FSharp.Systems

open Garnet.Composition
open Strategy.FSharp.Hexagon

[<Struct>]
type Update = { UpdateTime: float }

[<Struct>]
type Position = { X: float32; Y: float32 }

type GameState =
    | Startup
    | NewRound
    | Waiting
    | ContextMenu
    | Selected of Hexagon * Option<Eid>
