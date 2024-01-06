module Strategy.FSharp.Nodes

open Godot
open Garnet.Composition
open Strategy.FSharp.Systems

[<Struct>]
type Node = { NodeId: uint64 }

module NodesSystem =
    let registerUpdateNodePosition (c: Container) =
        c.On<PhysicsUpdate>
        <| fun _ ->
            for entity in c.Query<Node, Position>() do
                let node = entity.Value1
                let position = entity.Value2

                let node =
                    GodotObject.InstanceFromId node.NodeId :?> Node2D

                node.Position <- Vector2(position.X, position.Y)

    let register (c: Container) =
        Disposable.Create [ registerUpdateNodePosition c ]

    let findEntityWithNode (container: Container) (nodeId: uint64) =
        let foundEntity =
            container.Query<Eid, Node>()
            |> Seq.tryFind (fun entity -> entity.Value2.NodeId = nodeId)

        match foundEntity with
        | None -> None
        | Some entity -> Some(entity.Value1)
