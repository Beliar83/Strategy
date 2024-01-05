module Strategy.FSharp.GameWorld


open System
open System.Collections.Generic
open Godot
open Garnet.Composition
open Microsoft.FSharp.Collections
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
    let mutable physicsUpdate = Unchecked.defaultof<_>
    let mutable frameUpdate = Unchecked.defaultof<_>
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
        world.AddResource("MapRadius", 5)
        world.AddResource("UpdateMap", true)
        world.AddResource("CursorPosition", Vector2.Zero)
        world.AddResource("FieldsNeedUpdate", false)
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
            Map [ "Player1", { Color = ColorFromGodotColor(Colors.Red) }
                  "Player2", { Color = ColorFromGodotColor(Colors.Blue) } ]

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

        frameUpdate <-
            world.On<FrameUpdate>
            <| fun _ ->
                for entity in world.Query<Position, Node>() do
                    let node =
                        GodotObject.InstanceFromId(entity.Value2.NodeId) :?> Node2D

                    node.Position <- Vector2(entity.Value1.X, entity.Value1.Y)

                for entity in world.Query<UnitPosition, Node>() do
                    let node =
                        GodotObject.InstanceFromId(entity.Value2.NodeId) :?> UnitNode

                    node.SetBodyRotation(entity.Value1.BodyRotation)
                    node.SetWeaponRotation(entity.Value1.WeaponRotation)

        physicsUpdate <-
            world.On<PhysicsUpdate>
            <| fun _ ->
                let updateMap = world.LoadResource<bool>("UpdateMap")

                if updateMap then
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

                    let nextPlayer = playerQueue.Dequeue()
                    world.AddResource("CurrentPlayer", nextPlayer)
                    ChangeState Waiting world
                | Moving (eid, path) ->
                    if path.Length > 0 then
                        let entity = world.Get(eid)
                        let newCell = path.Head
                        let unit = entity.Get<Unit>()
                        let position = entity.Get<UnitPosition>()

                        let oldCell = position.Position

                        let bodyRotation =
                            if newCell = oldCell.GetNeighbour(Direction.NorthEast) then
                                30.0f
                            else if newCell = oldCell.GetNeighbour(Direction.East) then
                                90.0f
                            else if newCell = oldCell.GetNeighbour(Direction.SouthEast) then
                                150.0f
                            else if newCell = oldCell.GetNeighbour(Direction.SouthWest) then
                                210.0f
                            else if newCell = oldCell.GetNeighbour(Direction.West) then
                                270.0f
                            else if newCell = oldCell.GetNeighbour(Direction.NorthWest) then
                                330.0f
                            else
                                0f

                        entity.Set(
                            { Position = newCell
                              BodyRotation = bodyRotation
                              WeaponRotation = bodyRotation }
                        )

                        entity.Set(
                            { unit with
                                  RemainingRange = unit.RemainingRange - 1 }
                        )

                        ChangeState(Moving(eid, path.Tail)) world
                    else
                        world.AddResource("FieldsNeedUpdate", true)
                        ChangeState Waiting world
                        let position = world.Get(eid).Get<UnitPosition>()
                        world.AddResource<Option<Hexagon>>("CursorCell", Some(position.Position))
                        world.Send({ Button = Button.Select})
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
                  RemainingRange = 3
                  RemainingAttacks = 1 }
            )
            .With(
                { Position = Hexagon.NewAxial -1 -1
                  BodyRotation = 90.0f
                  WeaponRotation = 90.0f }
            )
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
                  RemainingRange = 3
                  RemainingAttacks = 1 }
            )
            .With(
                { Position = Hexagon.NewAxial -1 1
                  BodyRotation = 90.0f
                  WeaponRotation = 90.0f }
            )
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
                  RemainingRange = 3
                  RemainingAttacks = 0 }
            )
            .With(
                { Position = Hexagon.NewAxial 1 -1
                  BodyRotation = 270.0f
                  WeaponRotation = 270.0f }
            )
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
                  RemainingRange = 3
                  RemainingAttacks = 0 }
            )
            .With(
                { Position = Hexagon.NewAxial 1 1
                  BodyRotation = 270.0f
                  WeaponRotation = 270.0f }
            )
            .With({ PlayerId = "Player2" })
        |> ignore

        // First state needs to be set directly
        world.AddResource("State", GameState.NewRound)

    override this._Process(delta) = world.Run <| { FrameDelta = delta }

    override this._PhysicsProcess(delta) = world.Run <| { PhysicsDelta = delta }

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

    override this._Draw() =
        let state = world.LoadResource<GameState>("State")

        match state with
        | Startup
        | NewRound
        | Waiting -> ()
        | Selected (hexagon, _) ->
            let unitEntity =
                getEntitiesAtHexagon (hexagon, world)
                |> Array.map world.Get
                |> Array.choose
                    (fun entity ->
                        if entity.Has<Unit>() && entity.Has<Player>() then
                            Some(entity)
                        else
                            None)
                |> Array.tryHead

            match unitEntity with
            | None -> ()
            | Some unitEntity ->
                let currentPlayer =
                    world.LoadResource<string>("CurrentPlayer")

                let unitPlayer = unitEntity.Get<Player>()

                if unitPlayer.PlayerId = currentPlayer then
                    let unit = unitEntity.Get<Unit>()

                    let cell =
                        world.LoadResource<Option<Hexagon>>("CursorCell")

                    match cell with
                    | None -> ()
                    | Some cell ->
                        let path = findPath (hexagon, cell, world)

                        if path.Length <= unit.RemainingRange then
                            let mutable currentCell = hexagon

                            if path.Length > 0 then
                                for cell in path do
                                    let fromPosition = currentCell.Get2DPosition
                                    let toPosition = cell.Get2DPosition
                                    this.DrawDashedLine(fromPosition, toPosition, Colors.Black, 2.0f)
                                    currentCell <- cell
        | Moving _
        | ContextMenu -> ()
