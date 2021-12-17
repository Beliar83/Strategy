module Strategy.FSharp.GameWorld


open Godot
open Godot.Collections
open Garnet.Composition
open Strategy.FSharp.Hexagon
open Strategy.FSharp.HexMap
open Strategy.FSharp.Unit
open Strategy.FSharp.Input
open Strategy.FSharp.Nodes
open Strategy.FSharp.Systems


[<Struct>]
type Draw =
    struct

    end






let CreateGrid radius =

    let CreateCube (q, r) = Hexagon.NewAxial q r

    let InRadius (hexagon: Hexagon) =
        (hexagon.DistanceTo Hexagon.Zero) < radius

    let hexagons =
        let array = [| -radius .. radius + 1 |]

        array
        |> Array.collect (fun r -> array |> Array.map (fun s -> (r, s)))
        |> Array.map CreateCube
        |> Array.filter InRadius

    hexagons

type GameWorld() =
    inherit Node2D()
    let world = Container()
    let mutable update = Unchecked.defaultof<_>
    let cells = Dictionary<Hexagon, uint64>()

    override this._Ready() =
        world.AddResource("MapRadius", 1)
        world.AddResource("UpdateMap", true)
        world.AddResource("CursorPosition", Vector2.Zero)
        let unitsNode = this.GetNode(new NodePath("Units"))
        world.AddResource("UnitsNode", unitsNode.GetInstanceId())

        let cellsNode =
            new NodePath("Cells") |> this.GetNode :?> HexMap

        world.AddResource("CellsNode", cellsNode.GetInstanceId())

        let unitsNode =
            new NodePath("Units") |> this.GetNode :?> Node2D

        world.AddResource("UnitsNode", unitsNode.GetInstanceId())

        UnitSystem.register world |> ignore
        NodesSystem.register |> ignore
        HexMapSystem.register world |> ignore

        update <-
            world.On<Update>
            <| fun _ ->
                let update_map = world.LoadResource<bool>("UpdateMap")

                if update_map then
                    let mapRadius = world.LoadResource<int>("MapRadius")

                    let cellsNode =
                        new NodePath("Cells") |> this.GetNode :?> HexMap


                    cells.Clear()

                    let cells =
                        CreateGrid mapRadius
                        |> Array.map (fun c -> c.AsVector2)

                    cellsNode.Cells <- Array<Vector2>(cells)
                    world.Send { SelectedCell = None }

                    world.AddResource("UpdateMap", false)

        world
            .Create()
            .With(
                { Integrity = 10
                  Damage = 2
                  MaxAttackRange = 3
                  MinAttackRange = 1
                  Armor = 1
                  Mobility = 3
                  RemainingRange = 0
                  RemainingAttacks = 0 }
            )
            .With(Hexagon.NewAxial 0 0)
        |> ignore

        world.AddResource("State", GameState.Waiting)

    override this._PhysicsProcess(delta) = world.Run <| { UpdateTime = delta }

    override this._Input(event) =
        let camera =
            this.GetNode<Camera2D>(new NodePath "/root/Root/Camera2D")

        let event = camera.MakeInputLocal event

        match event with
        | :? InputEventKey as event ->
            let scancode = enum<KeyList> (int32 event.Scancode)

            if not event.Pressed then
                match scancode with
                | KeyList.Plus
                | KeyList.KpAdd ->
                    let radius = world.LoadResource<int> "MapRadius"
                    world.AddResource("MapRadius", radius + 1)
                    world.AddResource("UpdateMap", true)
                | KeyList.Minus
                | KeyList.KpSubtract ->
                    let radius = world.LoadResource<int> "MapRadius"
                    world.AddResource("MapRadius", max 1 (radius - 1))
                    world.AddResource("UpdateMap", true)
                | _ -> ()
        | :? InputEventMouseMotion as event -> world.AddResource("CursorPosition", event.Position)
        | :? InputEventMouseButton as event ->
            let button =
                enum<ButtonList> (int32 event.ButtonIndex)

            if not <| event.IsPressed() then
                match button with
                | ButtonList.Left -> world.Send <| { Button = Button.Select }
                | ButtonList.Right -> world.Send <| { Button = Button.Cancel }
                | _ -> ()

        | _ -> ()
