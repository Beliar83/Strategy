module Strategy.FSharp.DynamicNodes

open Godot
open Garnet.Composition
open Strategy.FSharp.Systems
open Strategy.FSharp.Nodes

type NodeTemplate = {
      NodePath: string;
      XScale: float32;
      YScale: float32;
      ZIndex: int;
      }


module DynamicNodeSystem =
      
      let registerCreateNodesFromTemplate (c: Container) =
            c.On<Update>
                  <| fun _ ->
                        let unitsNode = c.LoadResource<uint64>("UnitsNode")
                        let unitsNode = GD.InstanceFromId(unitsNode) :?> Node2D
                        for entity in c.Query<Eid, NodeTemplate>() do
                              let id = entity.Value1
                              let template = entity.Value2
                              let entity = c.Get id
                              if not <| entity.Has<Node>() then
                                    let node = GD.Load template.NodePath :?> PackedScene
                                    let node = node.Instance() :?> Node2D
                                    node.Scale <- Vector2(template.XScale, template.YScale)
                                    node.ZIndex <- template.ZIndex
                                    entity.Add {NodeId = node.GetInstanceId()}
                                    unitsNode.AddChild node

      let register (c: Container) =
            Disposable.Create [
                  registerCreateNodesFromTemplate c
            ]