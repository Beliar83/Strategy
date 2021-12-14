module Strategy.FSharp.Systems

[<Struct>]
type Update = { UpdateTime: float32 }

[<Struct>]
type Position = { X: float32; Y: float32 }

[<Struct>]
type Node = { NodeId: uint64 }
