module Strategy.FSharp.Unit

open Godot
open Garnet.Composition
open Strategy.FSharp.HexMap
open Strategy.FSharp.Hexagon
open Strategy.FSharp.Systems
open Strategy.FSharp.Nodes

type Unit =
    { Integrity: int32
      Damage: int32
      MaxAttackRange: int32
      MinAttackRange: int32
      Armor: int32
      Mobility: int32
      RemainingRange: int32
      RemainingAttacks: int32 }


type UnitNode() =
    inherit Node2D()

    let mutable integrity = 0
    let mutable cell = Hexagon.Zero
    let mutable selected = false

    member this.Integrity
        with get () = integrity
        and set value =
            let integrityLabel =
                this.GetNode(new NodePath("Integrity")) :?> Label

            integrityLabel.Text <- $"{this.Integrity}"
            integrity <- value

    member this.Cell
        with get () = cell
        and set value = cell <- value

    member this.Selected
        with get () = selected
        and set value =
            let outline =
                this.GetNode(new NodePath("Outline")) :?> Node2D

            outline.Visible <- value
            selected <- value

module UnitSystem =

    let registerUpdateUnitNodes (c: Container) =
        c.On<Update>
        <| fun _ ->
            let unitsNode = c.LoadResource<uint64>("UnitsNode")
            let unitsNode = GD.InstanceFromId(unitsNode) :?> Node2D

            for entity in c.Query<Eid, Unit, Hexagon>() do
                let id = entity.Value1
                let cell = entity.Value3

                let entity = c.Get id

                if not <| entity.Has<Node>() then
                    let node =
                        GD.Load "res://Unit.tscn" :?> PackedScene

                    let node = node.Instance() :?> UnitNode
                    entity.Add { NodeId = node.GetInstanceId() }
                    node.Position <- cell.Get2DPosition
                    unitsNode.AddChild node
                    entity.Add { NodeId = node.GetInstanceId() }

            for entity in c.Query<Unit, Node, Hexagon>() do
                let unit = entity.Value1
                let node = entity.Value2

                let node =
                    GD.InstanceFromId(node.NodeId) :?> UnitNode

                let cell = entity.Value3

                node.Integrity <- unit.Integrity
                node.Cell <- cell

    let registerInput (c: Container) =
        c.On<CellSelected>
        <| fun selected ->
            for entity in c.Query<Eid, Unit, Node, Hexagon>() do
                let id = entity.Value1
                let node = entity.Value3

                let node =
                    GD.InstanceFromId(node.NodeId) :?> UnitNode

                let cell = entity.Value4

                match selected.SelectedCell with
                | Some selected ->
                    if (selected = cell) then
                        node.Selected <- true
                        c.AddResource("State", GameState.Selected(cell, Some(id)))
                    else
                        node.Selected <- false
                | None -> node.Selected <- false

    let register (c: Container) =
        Disposable.Create [ registerUpdateUnitNodes c
                            registerInput c ]
