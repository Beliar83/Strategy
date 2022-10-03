module Strategy.FSharp.GameWorld


open System
open System.Collections.Generic
open Godot
open Garnet.Composition
open Strategy.FSharp.Hexagon
open Strategy.FSharp.HexMap
open Strategy.FSharp.MapUI
open Strategy.FSharp.Unit
open Strategy.FSharp.Input
open Strategy.FSharp.Nodes
open Strategy.FSharp.Systems
open Strategy.FSharp.Player

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
    let playerQueue = Queue<string>()

    let mutable mapUI: NodePath = null

    let mutable camera: NodePath = null

    member this.MapUI
        with get () = mapUI
        and set value = mapUI <- value

    member this.Camera
        with get () = camera
        and set value = camera <- value

    override this._Ready() =
        world.AddResource("MapRadius", 20)
        world.AddResource("UpdateMap", true)
        world.AddResource("CursorPosition", Vector2.Zero)
        world.AddResource("CurrentPlayer", "Player1")
        world.AddResource("WorldNode", this.GetInstanceId())
        let unitsNode = this.GetNode(new NodePath("Units"))
        world.AddResource("UnitsNode", unitsNode.GetInstanceId())

        let cellsNode = new NodePath("Cells") |> this.GetNode

        world.AddResource("CellsNode", cellsNode.GetInstanceId())

        let unitsNode =
            new NodePath("Units") |> this.GetNode :?> Node2D

        world.AddResource("UnitsNode", unitsNode.GetInstanceId())

        let players =
            Map [ "Player1", { Color = Colors.Red }
                  "Player2", { Color = Colors.Blue } ]

        world.AddResource("Players", players)

        let mapUI = this.GetNode(mapUI) :?> MapUI
        world.AddResource("UINode", mapUI.GetInstanceId())

        let camera = this.GetNode(camera) :?> Camera2D

        world.AddResource("Camera", camera.GetInstanceId())

        world.AddResource("CursorCell", Option<Hexagon>.None)
        world.AddResource("CellNodes", Map.empty<Hexagon, uint64>)

        UnitSystem.register world |> ignore
        NodesSystem.register |> ignore
        HexMapSystem.register world |> ignore
        MapUISystem.register world |> ignore

        update <-
            world.On<Update>
            <| fun _ ->
                let update_map = world.LoadResource<bool>("UpdateMap")

                if update_map then
                    let mapRadius = world.LoadResource<int>("MapRadius")

                    world.AddResource("Cells", CreateGrid mapRadius)
                    world.Send(CellsUpdated())

                    world.AddResource("UpdateMap", false)

                let state = world.LoadResource<GameState> "State"

                match state with
                | Waiting
                | Selected _ ->
                    let players =
                        world.LoadResource<Map<String, PlayerData>> "Players"

                    let playerId =
                        world.LoadResource<string> "CurrentPlayer"

                    let player = players.[playerId]
                    mapUI.SetPlayer(playerId, player)
                | NewRound ->
                    let players =
                        world.LoadResource<Map<String, PlayerData>> "Players"

                    if playerQueue.Count <= 0 then
                        for player in players do
                            playerQueue.Enqueue(player.Key)

                    let next_player = playerQueue.Dequeue()
                    world.AddResource("CurrentPlayer", next_player)
                    ChangeState Waiting world
                | _ -> ()

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
            .With(Hexagon.NewAxial -1 -1)
            .With({ PlayerId = "Player1" })
        |> ignore

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
            .With(Hexagon.NewAxial 1 1)
            .With({ PlayerId = "Player2" })
        |> ignore

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

        // First state needs to be set directly
        world.AddResource("State", GameState.NewRound)

    override this._PhysicsProcess(delta) = world.Run <| { UpdateTime = delta }

    override this._UnhandledInput(event) =
        let handleCursorMouseMotion (event: InputEventMouseMotion) =
            world.AddResource("CursorPosition", event.Position)

        let sendButtonFromMouseButton (event: InputEventMouseButton) =
            if not <| event.IsPressed() then
                match event.ButtonIndex with
                | MouseButton.Left -> world.Send <| { Button = Button.Select }
                | MouseButton.Right -> world.Send <| { Button = Button.Cancel }
                | _ -> ()

        let handleWaiting (event: InputEvent) =
            match event with
            | :? InputEventMouseMotion as event -> handleCursorMouseMotion event
            | :? InputEventMouseButton as event -> sendButtonFromMouseButton event
            | :? InputEventAction as event ->
                match event.Action.ToString() with
                | "ui_select" -> world.Send <| { Button = Button.Select }
                | "ui_cancel" -> world.Send <| { Button = Button.Cancel }
                | _ -> ()
            | _ -> ()

        let handleSelected (event: InputEvent) _ _ =
            match event with
            | :? InputEventMouseMotion as event -> handleCursorMouseMotion event
            | :? InputEventMouseButton as event -> sendButtonFromMouseButton event
            | :? InputEventAction as event ->
                match event.Action.ToString() with
                | "ui_select" -> world.Send <| { Button = Button.Select }
                | "ui_cancel" -> world.Send <| { Button = Button.Cancel }
                | _ -> ()
            | _ -> ()

        let camera =
            this.GetNode<Camera2D>(new NodePath "/root/Root/Camera2D")

        let event = camera.MakeInputLocal event

        match world.LoadResource<GameState> "State" with
        | Waiting -> handleWaiting event
        | Selected (cell, entity) -> handleSelected event cell entity
        | _ -> ()
