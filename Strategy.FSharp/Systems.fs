module Strategy.FSharp.Systems

open Garnet.Composition
open Strategy.FSharp.Hexagon

[<Struct>]
type Update = { UpdateTime: float32 }

[<Struct>]
type Position = { X: float32; Y: float32 }

type GameState =
    | Startup
    | NewRound
    | Waiting
    | Selected of Hexagon * Option<Eid>
