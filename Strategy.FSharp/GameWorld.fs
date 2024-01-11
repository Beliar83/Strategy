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

let setComponent(entity: Entity<int,Eid,EidSegmentKeyMapper>, comp : Object) =    
    let has = entity.GetType().GetMethod("Has").MakeGenericMethod(comp.GetType())
    let setComponent = 
        if has.Invoke(entity, null) :?> bool then
            entity.GetType().GetMethod("Set").MakeGenericMethod(comp.GetType())
        else
            entity.GetType().GetMethod("Add").MakeGenericMethod(comp.GetType())
    setComponent.Invoke(entity, [|comp|]) |> ignore


type GameWorld() =
    inherit Node2D()
    let world = Container()
    let mutable physicsUpdate = Unchecked.defaultof<_>
    let mutable frameUpdate = Unchecked.defaultof<_>
    let playerQueue = Queue<string>()
    let mutable mapUI: NodePath = null

    let mutable camera: NodePath = null
    let mutable map : NodePath = null
    
    let mutable entities : Map<Eid, List<Object>> = Map.empty
    
    member this.MapUI
        with get () = mapUI
        and set value = mapUI <- value

    member this.Camera
        with get () = camera
        and set value = camera <- value
    
    member this.Map
        with get() = map
        and set value =
            let gameState : GameState = GameState.Startup
            if world.TryGetResource<GameState>("State", ref gameState) then
                let map = this.GetNode(value) :?> HexMap
                map.World <- Some(world)
                
            map <- value

    member this.GetEntities() =
        entities
    
    member this.AddEntity() =
        let entity = world.Create()
        
        entities <- Map.add entity.Id List.empty entities
        entity.Id
    
    member this.RemoveEntity(entity: Eid) =
        world.Destroy(entity)
        entities <- Map.remove entity entities  

        
    member this.SetComponents(entity: Eid, components : List<Object>) =
        let entity = world.Get(entity)
        let mutable componentsToRemove = entities[entity.Id]
        
        // Reflection, but this should only be called in the editor, or once every map load,
        // and Garnet needs the generic methods to correctly store components.
        for comp in components do
            componentsToRemove <- componentsToRemove |> List.filter (fun c -> not <| c.Equals comp)
                
            setComponent(entity, comp)
            
        for comp in componentsToRemove do
            let remove = entity.GetType().GetMethod("Remove").MakeGenericMethod(comp.GetType())
            remove.Invoke(entity, null) |> ignore
            
        entities <- Map.add entity.Id components entities
        
    member this.SetComponent(entity: Eid, comp : Object) =
        let entity = world.Get(entity)
        setComponent(entity, comp)
    
    member this.Players
        with get () =        
            world.LoadResource<Map<string, PlayerData>>("Players")
        and set (value : Map<string, PlayerData>) =
            world.AddResource("Players", value)
        
    
    override this._Ready() =
        if not <| (map = null) then
            let map = this.GetNode(map) :?> HexMap
            map.World <- Some(world)
                
        world.AddResource("UpdateMap", false)
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
                        world.Send({ Button = Button.Select })
                | Attacking (attacker, attacked) ->
                    let attackerEntity = world.Get(attacker)
                    let attackedEntity = world.Get(attacked)
                    let attackerUnit = attackerEntity.Get<Unit>()
                    let attackedUnit = attackedEntity.Get<Unit>()

                    let damage = attackerUnit.Damage - attackedUnit.Armor
                    let remainingIntegrity = attackedUnit.Integrity - damage
                    let attackerPosition = attackerEntity.Get<UnitPosition>()
                    let attackedPosition = attackedEntity.Get<UnitPosition>()
                    
                    let angle = getAngleBetweenPositions(attackerPosition.Position, attackedPosition.Position)
                    
                    if attackerEntity.Has<Artillery>() then                    
                        attackerEntity.Set({ attackerPosition with BodyRotation = angle; WeaponRotation = angle })
                    else
                        attackerEntity.Set({ attackerPosition with WeaponRotation = angle })

                    attackedEntity.Set(
                        { attackedUnit with
                              Integrity = remainingIntegrity }
                    )

                    attackerEntity.Set(
                        { attackerUnit with
                              RemainingAttacks = attackerUnit.RemainingAttacks - 1 }
                    )

                    if remainingIntegrity <= 0 then
                        let unitNode =
                            GodotObject.InstanceFromId(attackedEntity.Get<Node>().NodeId) :?> UnitNode

                        unitNode.Destroy()
                        world.Destroy(attacked)

                    ChangeState(GameState.Selected(attackerPosition.Position, Some(attacker))) world
                | Startup ->
                    let players =
                        world.LoadResource<Map<String, PlayerData>> "Players"
                    if players.Count >= 1 then
                        ChangeState(GameState.NewRound) world                        

                | _ -> ()

        // First state needs to be set directly
        world.AddResource("State", GameState.Startup)

    override this._Process(delta) = world.Run <| { FrameDelta = delta }

    override this._PhysicsProcess(delta) =
        if not <| Engine.Singleton.IsEditorHint() then            
            world.Run <| { PhysicsDelta = delta }

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
        | _ -> ()
