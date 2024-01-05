module Strategy.FSharp.Unit

open Godot
open Garnet.Composition
open Strategy.FSharp.Hexagon
open Strategy.FSharp.Player
open Strategy.FSharp.Systems
open Strategy.FSharp.Nodes

let IsInMovementRange (unit: Unit, distance: int) =
    (distance > 0 && unit.RemainingRange >= distance)

type UnitNode() =
    inherit Node2D()

    let mutable integrity = 0
    let mutable cell = Hexagon.Zero
    let mutable selected = false
    let mutable bodyNode: NodePath = Unchecked.defaultof<_>
    let mutable weaponNode: NodePath = Unchecked.defaultof<_>

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

    member this.BodyNode
        with get () = bodyNode
        and set value = bodyNode <- value

    member this.SetColor(color: Color) =
        let body = this.GetNode(bodyNode) :?> CanvasItem
        let material = body.Material :?> ShaderMaterial
        material.SetShaderParameter("color", GodotColorFromColor(color))

        let weapon =
            body.GetNode(new NodePath("Weapon")) :?> CanvasItem

        let material = weapon.Material :?> ShaderMaterial
        material.SetShaderParameter("color", GodotColorFromColor(color))

    member this.SetBodyRotation(rotation: float32) =
        let body = this.GetNode(bodyNode) :?> Body
        body.SetBodyRotation(rotation)

    member this.SetWeaponRotation(rotation: float32) =
        let body = this.GetNode(bodyNode) :?> Body
        body.SetWeaponRotation(rotation)

module UnitSystem =

    let registerUpdateUnitNodes (c: Container) =
        c.On<PhysicsUpdate>
        <| fun _ ->
            let unitsNode = c.LoadResource<uint64>("UnitsNode")

            let unitsNode =
                GodotObject.InstanceFromId(unitsNode) :?> Node2D

            let players =
                c.LoadResource<Map<string, PlayerData>>("Players")

            for entity in c.Query<Eid, Unit, UnitPosition>() do
                let id = entity.Value1
                let cell = entity.Value3.Position

                let entity = c.Get id

                if not <| entity.Has<Node>() then
                    let node =
                        GD.Load "res://Unit.tscn" :?> PackedScene

                    let node = node.Instantiate() :?> UnitNode
                    entity.Add { NodeId = node.GetInstanceId() }
                    node.Position <- cell.Get2DPosition

                    if entity.Has<Player>() then
                        let player = entity.Get<Player>()
                        node.SetColor(players[player.PlayerId].Color)

                    unitsNode.AddChild node
                    entity.Add { NodeId = node.GetInstanceId() }

            for entity in c.Query<Eid, Unit, Node, UnitPosition>() do
                let id = entity.Value1
                let unit = entity.Value2
                let node = entity.Value3

                let node =
                    GodotObject.InstanceFromId(node.NodeId) :?> UnitNode

                let cell = entity.Value4.Position

                node.Integrity <- unit.Integrity
                node.Cell <- cell

    let rec updateSelection (container: Container) =
        let state =
            container.LoadResource<GameState>("State")

        let isSelected =
            match state with
            | GameState.Selected (_, entity) ->
                match entity with
                | Some selectedId -> fun id -> id = selectedId
                | None -> fun _ -> false
            | _ -> fun _ -> false

        for entity in container.Query<Eid, Unit, Node, UnitPosition>() do
            let id = entity.Value1
            let node = entity.Value3

            let node =
                GodotObject.InstanceFromId(node.NodeId) :?> UnitNode

            node.Selected <- isSelected id

    let registerSelectCell (container: Container) =
        container.On<SelectCell>
        <| fun _ -> updateSelection container

    let registerDeselectCell (container: Container) =
        container.On<DeselectCell>
        <| fun _ -> updateSelection container

    let register (c: Container) =
        Disposable.Create [ registerUpdateUnitNodes c
                            registerSelectCell c
                            registerDeselectCell c ]
