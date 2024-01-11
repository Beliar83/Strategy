module Strategy.FSharp.HexMap

open System.Collections.Generic
open Godot
open Microsoft.FSharp.Collections
open Microsoft.FSharp.Core
open Strategy.FSharp.Hexagon
open Garnet.Composition
open Strategy.FSharp.Input
open Strategy.FSharp.Nodes
open Strategy.FSharp.Player
open Strategy.FSharp.Systems
open Strategy.FSharp.Unit

let PolygonWidth cellSize = sqrt 3f * cellSize

let PolygonHeight cellSize = 2f * cellSize

let Half value = value / 2f
let Quarter value = value / 4f

let GROUND_BIT = 0
let UNIT_BIT = 1;

[<Struct>]
type CellsUpdated = struct end

let HexagonPoints cellSize =
    let width = PolygonWidth cellSize
    let height = PolygonHeight cellSize
    let halfHeight = Half height
    let quarterHeight = Quarter height
    let halfWidth = Half width

    [| Vector2(-halfWidth, -quarterHeight)
       Vector2(0f, -halfHeight)
       Vector2(halfWidth, -quarterHeight)
       Vector2(halfWidth, quarterHeight)
       Vector2(0f, halfHeight)
       Vector2(-halfWidth, quarterHeight)
       Vector2(-halfWidth, -quarterHeight) |]

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

type HexMap() =
    inherit Node2D()
    
    let mutable world : Option<Container> = None
    let mutable mapRadius = 0
    
    member this.World
        with get () = world
        and set value =
            world <- value
            match value with
            | None -> ()
            | Some world -> world.Send(CellsUpdated())
    
    member this.MapRadius
        with get () = mapRadius
        and set value =
            mapRadius <- value            
            match world with
            | None -> ()
            | Some world ->
                world.Send(CellsUpdated())       
    
    member this.UpdateCells(container: Container) =
        container.AddResource("MapRadius", mapRadius)

        container.AddResource("Cells", CreateGrid mapRadius)

        while this.GetChildCount() > 0 do
            let node = this.GetChildOrNull<Godot.Node> 0

            if not <| isNull node then
                node.QueueFree()
                this.RemoveChild node

        let hexagon =
            GD.Load("res://Hexagon.tscn") :?> PackedScene
        

        let cells = container.LoadResource<Hexagon[]>("Cells")
        
        let cellNodes =
            cells
            |> Array.map (fun cell ->
                let node = hexagon.Instantiate() :?> Node2D
                let cell = Hexagon.FromVector2 cell.AsVector2
                node.Position <- cell.Get2DPosition
                node.Set("Cell", Vector3(float32 cell.Q, float32 cell.R, float32 cell.S))
                this.AddChild node
                (cell, node.GetInstanceId()) )
        
        let cellNodes = Map.ofArray cellNodes
        
        container.AddResource("CellNodes", cellNodes)

    member this.GetCellAtPosition(position: Vector2) = Hexagon.At2DPosition position


let GetNeighbours (hexagon: Hexagon) =
    [| hexagon.GetNeighbour(Direction.East),
       hexagon.GetNeighbour(Direction.NorthEast),
       hexagon.GetNeighbour(Direction.NorthWest),
       hexagon.GetNeighbour(Direction.West),
       hexagon.GetNeighbour(Direction.SouthWest),
       hexagon.GetNeighbour(Direction.SouthEast) |]

[<Struct>]
type UpdateSelectedCell = {Cell: Hexagon}

let GetCellAtPosition(position: Vector2) = Hexagon.At2DPosition position

let emitCellSignal signal cell (cellNodes : Map<Hexagon, uint64>) =
    let node = (GodotObject.InstanceFromId cellNodes[cell])
    node.EmitSignal signal

let emitCellSelected =
    emitCellSignal (new StringName "selected")

let emitCellDeselected =
    emitCellSignal (new StringName "deselected")

let emitCursorEnteredCell =
    emitCellSignal (new StringName "cursor_entered")

let emitCursorExitedCell =
    emitCellSignal (new StringName "cursor_exited")
let emitHighlightMovable =
    emitCellSignal (new StringName "highlight_movable")
let emitUnhighlightMovable =
    emitCellSignal (new StringName "unhighlight_movable")
let emitHighlightAttackable =
    emitCellSignal (new StringName "highlight_attackable")
let emitUnhighlightAttackable =
    emitCellSignal (new StringName "unhighlight_attackable")

let UpdateCursorCell(cell: Hexagon) (c: Container) =
        let cursorCell = c.LoadResource<Option<Hexagon>>("CursorCell")
        let sameCell =
            match cursorCell with
            | Some currentCell -> cell = currentCell
            | None -> false

        let cellNodes = c.LoadResource<Map<Hexagon, uint64>>("CellNodes")

        if not sameCell then
            match cursorCell with
            | Some currentCell ->
                if cellNodes.ContainsKey(currentCell) then
                    emitCursorExitedCell currentCell cellNodes |> ignore
            | None -> ()

            if cellNodes.ContainsKey(cell) then
                c.AddResource("CursorCell", Some(cell))
                emitCursorEnteredCell cell cellNodes |> ignore
            else
                c.AddResource("CursorCell", Option<Hexagon>.None)
            
            let worldNode = GodotObject.InstanceFromId(c.LoadResource<uint64>("WorldNode")) :?> Node2D            
            worldNode.QueueRedraw()

let getEntitiesAtHexagon(cell: Hexagon, container : Container) =
    let position = cell.Get2DPosition
    container.Query<Eid, UnitPosition>()
    |> Seq.filter (fun query -> query.Value2.Position = cell)
    |> Seq.map (_.Value1)
    |> Array.ofSeq
    

let resetCells (c: Container) =
    let cellNodes = c.LoadResource<Map<Hexagon,uint64>> "CellNodes"
    for cell in cellNodes.Keys do
        emitCellSignal "unhighlight_movable" cell cellNodes |> ignore
        emitCellSignal "unhighlight_attackable" cell cellNodes |> ignore

let getNeighbours(hexagon: Hexagon) =
    [|hexagon.GetNeighbour(Direction.East);
      hexagon.GetNeighbour(Direction.NorthEast);
      hexagon.GetNeighbour(Direction.NorthWest);
      hexagon.GetNeighbour(Direction.West);
      hexagon.GetNeighbour(Direction.SouthWest);
      hexagon.GetNeighbour(Direction.SouthEast);
      |]

let findPath(start : Hexagon, target : Hexagon, container: Container) =
    if getEntitiesAtHexagon(target, container)
        |> Seq.exists(fun eid ->
            let entity = container.Get(eid)
            entity.Has<Unit>()
            )
        then List.Empty
    else
        let mutable frontier = PriorityQueue<int, Hexagon>();
        frontier.Enqueue(0, start)
        let mutable cameFrom = Dictionary<Hexagon, Option<Hexagon>>()
        let mutable costSoFar = Dictionary<Hexagon, int>()
        cameFrom[start] <- None
        costSoFar[start] <- 0
        while frontier.Count > 0 do
            let current = frontier.Dequeue()
            if current = target then frontier.Clear()
            else
                let neighbours = getNeighbours(current)
                for neighbour in neighbours do
                    if getEntitiesAtHexagon(neighbour, container)
                        |> Seq.exists(fun eid ->
                        let entity = container.Get(eid)
                        entity.Has<Unit>()
                        ) then ()
                    else
                        let newCost = costSoFar[current] + 1
                        if not <| costSoFar.ContainsKey(neighbour) || newCost < costSoFar[neighbour] then
                            costSoFar[neighbour] <- newCost
                            let priority = newCost + neighbour.DistanceTo(target)
                            frontier.Enqueue(priority, neighbour)
                            cameFrom[neighbour] <- Some(current)
        
        let mutable path = List.Empty
        if cameFrom[target].IsSome then
            let mutable current = cameFrom[target].Value            
            path <- path |> List.insertAt 0 target
            while not <| (current = start) do
                path <- path |> List.insertAt 0 current
                current <- cameFrom[current].Value
        path

module HexMapSystem =
    let registerUpdatePosition (c: Container) =
        c.On<FrameUpdate>
        <| fun _ ->
            for entity in c.Query<Eid, UnitPosition>() do
                let entityId = entity.Value1
                let hexagon = entity.Value2.Position
                let entity = c.Get entityId
                let position = hexagon.Get2DPosition
                entity.Add { X = position.X; Y = position.Y }    
    
    let isPlayerOfEntitySameAsPlayer (entity : Entity<int,Eid,EidSegmentKeyMapper>, player: Player) =
                            if entity.Has<Player.Player>() then
                                let entityPlayer = entity.Get<Player.Player>()                            
                                player.PlayerId = entityPlayer.PlayerId
                            else
                                false
    let canAttackCellFromCell (unitEntityId : Eid) (container: Container) (attackFrom : Hexagon) (toAttack : Hexagon) =
        let cellNodes = container.LoadResource<Map<Hexagon,uint64>> "CellNodes"
        let currentPlayerId = container.LoadResource<string>("CurrentPlayer")
        let unitEntity = container.Get(unitEntityId)
        let unitNode = GodotObject.InstanceFromId(unitEntity.Get<Node>().NodeId) :?> Node2D
        let unit = unitEntity.Get<Unit>()
        if unitEntity.Has<Player.Player>() then
            let selectedCellPlayer = unitEntity.Get<Player.Player>()
            let distanceToUnit = toAttack.DistanceTo(attackFrom)
            let doesSelectedUnitBelongToCurrentPlayer = selectedCellPlayer.PlayerId = currentPlayerId
            if doesSelectedUnitBelongToCurrentPlayer then
                if distanceToUnit <= unit.RemainingRange then
                    let canMove = IsInMovementRange(unit, findPath(attackFrom, toAttack, container).Length) 
                    if canMove then
                        emitHighlightMovable toAttack cellNodes |> ignore
            if (unit.RemainingAttacks > 0 || not <| doesSelectedUnitBelongToCurrentPlayer) && distanceToUnit >= unit.MinAttackRange && distanceToUnit <= unit.MaxAttackRange then
                let entitiesAtTarget = getEntitiesAtHexagon(toAttack, container)
                let isTargetSamePlayer =
                    entitiesAtTarget
                    |> Array.exists (fun eid ->                                    
                            let entity = container.Get(eid)
                            if entity.Has<Unit>() then
                                isPlayerOfEntitySameAsPlayer(entity, selectedCellPlayer)
                            else
                                false
                        )
                let canArtilleryAttack() =
                    let angle = getAngleBetweenPositions(attackFrom, toAttack)
                    let currentRotation = unitEntity.Get<UnitPosition>().BodyRotation
                    Mathf.Abs(angle - currentRotation) <= 30.0f
                    
                let canTankAttack() =
                    let selfPosition = attackFrom.Get2DPosition
                    let cellPosition = toAttack.Get2DPosition
                    let mapSize = container.LoadResource<int>("MapRadius")                   
                    
                    let adjustmentVector = Vector2((mapSize |> float32) / 8.0f, (mapSize |> float32 ) / 8.0f)
                    let rightSideRayCast() =                  
                        let queryParameters = new PhysicsRayQueryParameters2D()
                        queryParameters.From <- selfPosition + adjustmentVector
                        queryParameters.To <- cellPosition
                        queryParameters.CollisionMask <- 1u <<< UNIT_BIT
                        queryParameters.CollideWithAreas <- true
                        queryParameters.HitFromInside <- false                    
                        
                        unitNode.GetWorld2D().DirectSpaceState.IntersectRay(queryParameters)

                    let leftSideRayCast() =
                        let queryParameters = new PhysicsRayQueryParameters2D()
                        queryParameters.From <- selfPosition - adjustmentVector
                        queryParameters.To <- cellPosition
                        queryParameters.CollisionMask <- 1u <<< UNIT_BIT
                        queryParameters.CollideWithAreas <- true
                        queryParameters.HitFromInside <- false                    
                        
                        unitNode.GetWorld2D().DirectSpaceState.IntersectRay(queryParameters)
                    
                    let result = rightSideRayCast()
                                        
                    if result.Count = 0 then
                        true
                    else
                        let result = leftSideRayCast()
                        if result.Count = 0 then
                            true
                        else
                            let hitId = result["collider_id"].AsUInt64()
                            let hitNode = GodotObject.InstanceFromId(hitId) :?> Node2D
                            let hitHexagon = Hexagon.At2DPosition(hitNode.Position)
                            hitHexagon = toAttack                
                
                if not <| isTargetSamePlayer then
                    if unitEntity.Has<Artillery>() then
                        canArtilleryAttack()
                    else
                        canTankAttack()
                else
                    false
            else
                false
        else
            false
    
    let registerUpdateMap (c: Container) =
        c.On<FrameUpdate>
        <| fun _ ->

            let mousePosition = c.LoadResource<Vector2> "CursorPosition"

            let mouseCell = GetCellAtPosition mousePosition

            c.Send <| { CursorCell = mouseCell }
            
            let fieldsNeedUpdate = c.LoadResource<bool> "FieldsNeedUpdate"
            
            if fieldsNeedUpdate then
                let state = c.LoadResource<GameState> "State"
                resetCells c
                match state with
                | Selected(hexagon, eidOption) ->
                    match eidOption with
                    | None -> ()
                    | Some eid ->                        
                        let canAttack = canAttackCellFromCell eid c hexagon
                        let cells = c.LoadResource<array<Hexagon>> "Cells"
                        let cellNodes = c.LoadResource<Map<Hexagon,uint64>> "CellNodes"
                        for cell in cells do
                            if canAttack cell then
                                emitHighlightAttackable cell cellNodes |> ignore
                | _ -> ()

                c.AddResource("FieldsNeedUpdate", false)

    let registerCursorEntered (c: Container) =
        c.On<CursorMoved>
        <| fun event ->
            UpdateCursorCell event.CursorCell c

    let registerSelectPressed (c: Container) =

        let handleSelect () =
            let state = c.LoadResource<GameState>("State")

            let getEntitiesAtCell (cell: Hexagon) =
                c.Query<Eid, UnitPosition>()
                |> Seq.filter (fun x -> x.Value2.Position = cell)
                |> Seq.map (fun x -> c.Get x.Value1)
                |> Seq.toArray

            let showContextMenuForCell (cell: Hexagon) =
                let items = [||]

                let entities = getEntitiesAtCell cell

                let entityCommand entityId =
                    ChangeState (GameState.Selected(cell, Some(entityId))) c

                let moveCommand(entityId, path) =
                    ChangeState (GameState.Moving(entityId, path)) c
                
                let attackCommand(attacker : Eid, attacked: Eid) =
                    ChangeState (GameState.Attacking(attacker, attacked)) c
                
                let getItemForUnit (entity: Entity) (_unit: Unit) =
                    { Label = "Unit"
                      Command = (fun () -> entityCommand entity.Id)
                      ItemType = ItemType.IconItem("res://assets/units/tank.png") }
                
                let entitiesWithUnit =
                    entities
                    |> Array.filter (fun entity -> entity.Has<Unit>())
 
                
                let items =
                    entitiesWithUnit
                    |> Array.map (fun entity -> getItemForUnit entity (entity.Get<Unit>()))
                    |> Array.append items

                let items =
                    Array.append
                        items
                        [|
                            let currentUnitEntity =
                                match state with
                                | Startup
                                | NewRound
                                | Waiting -> None
                                | Selected(_, eidOption) ->
                                    match eidOption with
                                    | None -> None
                                    | Some eid ->
                                        let entity = c.Get(eid)
                                        if entity.Has<Unit>() && entity.Has<UnitPosition>() then
                                            Some(entity)
                                        else
                                            None
                                | Moving _ -> None
                                | ContextMenu _ -> None
                                | Attacking _ -> None
                            match currentUnitEntity with
                            | None -> ()
                            | Some entity ->
                                let unit = entity.Get<Unit>()
                                let unitPosition = entity.Get<UnitPosition>()
                                let path = findPath(unitPosition.Position, cell, c)
                                if IsInMovementRange(unit, path.Length) then
                                    { Label = "Move"; Command = (fun () -> moveCommand(entity.Id, path)); ItemType = ItemType.IconItem("res://assets/icons/move.png") }
                                else
                                    let targetEntity =
                                        getEntitiesAtCell(cell)
                                        |> Array.filter(fun entity -> entity.Has<Unit>())
                                        |> Array.tryHead

                                    match targetEntity with
                                    | None -> ()
                                    | Some targetEntity ->                                    

                                        if canAttackCellFromCell entity.Id c unitPosition.Position cell then
                                            { Label = "Attack"; Command = (fun() -> attackCommand(entity.Id, targetEntity.Id)); ItemType = ItemType.IconItem("res://assets/icons/attack.png") }
                                        else
                                            ()
                        |]
                
                let position = cell.Get2DPosition

                let camera = c.LoadResource<uint64> "Camera"
                let camera = GodotObject.InstanceFromId camera :?> Camera2D
                let rect = camera.GetViewportRect()
                let halfSize = rect.Size / 2f
                let position = position + halfSize

                let endTurn () =
                    ChangeState GameState.NewRound c

                let close () =
                    c.AddResource("CursorPosition", camera.GetLocalMousePosition())
                    ChangeState Waiting c

                let items =
                    Array.append
                        items
                        [| { Label = "End Turn"
                             Command = endTurn
                             ItemType = ItemType.IconItem "res://assets/icons/simpleBlock.png" } |]

                let items =
                    Array.append
                        items
                        [| { Label = "Close"
                             Command = close
                             ItemType = ItemType.Item } |]

                let state = c.LoadResource<GameState>("State")
                ChangeState (GameState.ContextMenu(state)) c                
                
                c.Run {Items = Array.toList items; Position = Vector2I.op_Explicit position; ClosedHandler = close}

            match state with
            | GameState.Selected _
            | GameState.Waiting ->
                let cursorCell = c.LoadResource<Option<Hexagon>>("CursorCell")
                match cursorCell with
                | Some cursorCell ->
                    showContextMenuForCell cursorCell
                | None -> ()
            | _ -> ()

        let handleCancel () =
            match c.LoadResource<GameState>("State") with
            | Selected (cell, _) ->
                c.Send { DeselectedCell = cell }
                ChangeState Waiting c
            | _ -> ()

        c.On<ButtonPressed>
        <| fun event ->
            match event.Button with
            | Select -> handleSelect ()
            | Cancel -> handleCancel ()
    
    let registerCellSelected (container: Container) =                
        container.On<SelectCell>
        <| fun event ->
            let cellNodes = container.LoadResource<Map<Hexagon, uint64>>("CellNodes")
            if cellNodes.ContainsKey(event.SelectedCell) then
                emitCellSelected event.SelectedCell cellNodes |> ignore                
    
    let registerCellDeselected (container: Container) =
        container.On<DeselectCell>
        <| fun event ->
            let cellNodes = container.LoadResource<Map<Hexagon, uint64>>("CellNodes")
            if cellNodes.ContainsKey(event.DeselectedCell) then
                emitCellDeselected event.DeselectedCell cellNodes |> ignore

    let registerCellsUpdated (container :Container) =
        container.On<CellsUpdated>
        <| fun _ ->
            let hexMap = container.LoadResource<uint64>("CellsNode")
            let hexMap = GodotObject.InstanceFromId hexMap :?> HexMap
            hexMap.UpdateCells container
    
    let register (c: Container) =
        Disposable.Create [ registerUpdatePosition c
                            registerUpdateMap c
                            registerCursorEntered c
                            registerSelectPressed c
                            registerCellSelected c
                            registerCellDeselected c
                            registerCellsUpdated c]