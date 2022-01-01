module Strategy.FSharp.Unit

open System
open Godot
open Garnet.Composition
open Strategy.FSharp.Hexagon
open Strategy.FSharp.Input
open Strategy.FSharp.Player
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

    member this.Color
        with get () =
            (this.GetNode(new NodePath("Model")) :?> CanvasItem)
                .Modulate
        and set value = (this.GetNode(new NodePath("Model")) :?> CanvasItem).Modulate <- value

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

            for entity in c.Query<Eid, Unit, Node, Hexagon>() do
                let id = entity.Value1
                let unit = entity.Value2
                let node = entity.Value3

                let node =
                    GD.InstanceFromId(node.NodeId) :?> UnitNode

                let cell = entity.Value4

                node.Integrity <- unit.Integrity
                node.Cell <- cell

                let entity = c.Get id

                if entity.Has<Player>() then
                    let player = entity.Get<Player>()

                    let players =
                        c.LoadResource<Map<String, PlayerData>> "Players"

                    if players.ContainsKey(player.PlayerId) then
                        node.Color <- players.[player.PlayerId].Color
                else
                    node.Color <- Color.ColorN("Gray")

    let registerInput (c: Container) =

        c.On<UpdateSelection>
        <| fun _ ->
            let state = c.LoadResource<GameState>("State")

            let is_selected =
                match state with
                | GameState.Selected (_, entity) ->
                    match entity with
                    | Some selected_id -> fun id -> id = selected_id
                    | None -> fun _ -> false
                | _ -> fun _ -> false

            for entity in c.Query<Eid, Unit, Node, Hexagon>() do
                let id = entity.Value1
                let node = entity.Value3

                let node =
                    GD.InstanceFromId(node.NodeId) :?> UnitNode

                node.Selected <- is_selected id

    let register (c: Container) =
        Disposable.Create [ registerUpdateUnitNodes c
                            registerInput c ]
