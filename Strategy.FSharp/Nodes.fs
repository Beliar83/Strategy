module Strategy.FSharp.Nodes

open Godot
open Garnet.Composition
open Strategy.FSharp.Systems

[<Struct>]
type Node = { NodeId: uint64 }

module NodesSystem =
    let registerUpdateNodePosition (c: Container) =
        c.On<Update>
             <| fun _ ->
                for entity in c.Query<Node, Position>() do
                let node = entity.Value1
                let position = entity.Value2
                
                let node = GD.InstanceFromId node.NodeId :?> Node2D
                node.Position <- Vector2(position.X, position.Y)
    let register (c: Container) =
        Disposable.Create [
            registerUpdateNodePosition c
        ]