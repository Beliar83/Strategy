module Strategy.FSharp.GameWorld


open Godot
open Godot.Collections
open Garnet.Composition
open Strategy.FSharp.Hexagon
open Strategy.FSharp.HexMap

[<Struct>]
type Position = { X: float32; Y: float32 }

[<Struct>]
type Node = { id: uint64 }

[<Struct>]
type Update = { UpdateTime: float32 }

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
        world.AddResource("UpdateSelected", true)

        update <-
            world.On<Update>
            <| fun time ->
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

                    world.AddResource("UpdateMap", false)
                    world.AddResource("UpdateSelected", true)

                let updateSelected =
                    world.LoadResource<bool> "UpdateSelected"

                if updateSelected then
                    let mousePosition =
                        world.LoadResource<Vector2> "CursorPosition"

                    let cellsNode =
                        new NodePath("Cells") |> this.GetNode :?> HexMap

                    let cell =
                        cellsNode.GetCellAtPosition mousePosition

                    cellsNode.SelectCell cell
                    world.AddResource("UpdateSelected", false)


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
        | :? InputEventMouseMotion as event ->
            world.AddResource("CursorPosition", event.Position)
            world.AddResource("UpdateSelected", true)
        | _ -> ()
