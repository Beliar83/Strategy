using System;
using System.Collections.Generic;
using System.Linq;
using Godot;
using Godot.Collections;
using Strategy.Components;

namespace Strategy;

[Tool]
[GlobalClass]
public partial class GameWorld : Node2D
{
    private const string NewRoundIconPath = "uid://hx5istoogfpj";

    [Export]
    public int UnitMask { get; set; } = 1;

    private Array<PlayerData?> players = [];
    private System.Collections.Generic.Dictionary<StringName, PlayerData>? playersByName;

    private readonly StringName playersName = new("Players");
    
    private readonly Queue<StringName> playerQueue = new();
    private HexMap? map;

    public float cellSize = 1;

    public delegate void PlayerChanged(StringName playerId);
    
    public event PlayerChanged? OnPlayerChanged;

    public GameState GameState { get; private set; } = new GameState.Startup();

    [Export(PropertyHint.NodeType, nameof(MapUI))]
    public MapUI? MapUI { get; set; }

    [Export(PropertyHint.NodeType, nameof(Camera2D))]
    public Camera2D? Camera { get; set; }

    [Export(PropertyHint.NodeType, nameof(HexMap))]
    public HexMap? Map
    {
        get => map;
        set
        {
            map = value;
            if (map?.TileSet is not null)
            {
                cellSize = map.TileSet.TileSize.X / 2f;
            }
        }
    }

    [Export]
    public Array<PlayerData?> Players
    {
        get => players;

        set
        {
            if (value.OfType<PlayerData>().Where(p => !string.IsNullOrWhiteSpace(p.Name)).GroupBy(p => p.Name).Any(g => g.Count() > 1))
            {
                GD.PrintErr("Could not set players: Duplicate names found.");
                return;
            }

            players = value;
            playersByName = null;
      }
    }

    public System.Collections.Generic.Dictionary<StringName, PlayerData> PlayersByName
    {
        get
        {
            return playersByName ??= players.OfType<PlayerData>().Where(p => !string.IsNullOrWhiteSpace(p.Name)).ToDictionary(p => new StringName(p.Name));
        }
    }

    private StringName? CurrentPlayer { get; set; }

    private GameState GetChangedState(GameState currentGameState, GameState newGameState)
    {
        return currentGameState switch
        {
            GameState.Startup => ChangeStartFromStartup(),
            GameState.Waiting => ChangeFromWaiting(),
            GameState.Selected state => ChangeFromSelected(state.SelectedUnit),
            GameState.NewRound => ChangeFromNewRound(),
            GameState.Moving state => ChangeFromMoving(state.Unit, state.Path),
            GameState.Attacking => ChangeFromAttacking(),
            GameState.ContextMenu state => ChangeFromContextMenu(state.StoredState),
            _ => throw new ArgumentOutOfRangeException(nameof(currentGameState)),
        };

        GameState ChangeStartFromStartup()
        {
            return newGameState switch
            {
                GameState.NewRound => newGameState,
                _ => currentGameState,
            };
        }

        GameState ChangeFromWaiting()
        {
            switch (newGameState)
            {
                case GameState.Startup:
                case GameState.ContextMenu:
                case GameState.Waiting:
                    return newGameState;
                case GameState.Selected selected:
                    SelectCell(selected.Cell);
                    return newGameState;
                case GameState.Moving:
                case GameState.Attacking:
                    return currentGameState;
                case GameState.NewRound:
                    return newGameState;
                default:
                    throw new ArgumentOutOfRangeException(nameof(newGameState));
            }
        }

        GameState ChangeFromSelected(Node? selected)
        {
            switch (newGameState)
            {
                case GameState.NewRound:
                    ClearSelection();
                    return newGameState;
                case GameState.Attacking state:
                    if (selected is not null)
                    {
                        return selected == state.Attacker 
                            ? newGameState 
                            : currentGameState;
                    }
                    return currentGameState;
                case GameState.Selected state:
                    SelectCell(state.Cell);
                    return newGameState;
                default:
                    ClearSelection();
                    return newGameState;
            }
        }

        GameState ChangeFromNewRound()
        {
            return newGameState switch
            {
                GameState.Waiting => newGameState,
                _ => currentGameState,
            };
        }

        GameState ChangeFromMoving(Unit currentUnit, List<Hexagon> currentPath)
        {
            switch (newGameState)
            {
                case GameState.Waiting:
                {
                    return currentPath.Count <= 0 
                        ? newGameState 
                        : currentGameState;
                }
                case GameState.Moving state:
                {
                    if (state.Unit == currentUnit && state.Path.Count < currentPath.Count)
                    {
                        return newGameState;
                    }

                    return currentGameState;
                }
                case GameState.Selected state:
                {
                    if (state.SelectedUnit == currentUnit)
                    {
                        SelectCell(state.Cell);
                        return newGameState;
                    }
                    else
                    {
                        return currentGameState;
                    }
                }
                default:
                    return currentGameState;
            }
        }

        GameState ChangeFromAttacking()
        {
            switch (newGameState)
            {
                case GameState.Attacking:
                case GameState.Waiting:
                    return newGameState;
                case GameState.Selected state:
                    SelectCell(state.Cell);
                    return newGameState;
                default:
                    return currentGameState;
            }
        }

        GameState ChangeFromContextMenu(GameState storedState)
        {
            return GetChangedState(storedState, newGameState);
        }
    }

    public void ChangeState(GameState newGameState)
    {
        GameState = GetChangedState(GameState, newGameState);
    }

    /// <inheritdoc />
    public override void _PhysicsProcess(double delta)
    {
        switch (GameState)
        {
            case GameState.Waiting:
            case GameState.Selected:
                break;
            case GameState.NewRound:
                HashSet<StringName> availablePlayers = PlayersByName.Keys.ToHashSet();
                if (availablePlayers.Count <= 0)
                {
                    break;
                }
                
                if (playerQueue.Count <= 0)
                {
                    foreach (StringName playerId in availablePlayers)
                    {
                        playerQueue.Enqueue(playerId);
                    }
                }
                
                CurrentPlayer = playerQueue.Dequeue();
                foreach (Node node in Component.GetInstancesOfComponent<Player>(GetTree()).Where(p => p.PlayerId == CurrentPlayer).Select(p => p.GetParent()))
                {
                    foreach (Unit unit in node.GetChildren().OfType<Unit>())
                    {
                        unit.RemainingAttacks = 1;
                        unit.RemainingRange = unit.Mobility;
                    }                    
                }

                OnPlayerChanged?.Invoke(CurrentPlayer);
                ChangeState(new GameState.Waiting());
                break;
            case GameState.Moving state:
                HexagonNode2D entityNode = state.Unit.GetParent<HexagonNode2D>();
                if (state.Path.Count > 0)
                {
                    Hexagon newCell = state.Path.First();
                    if (newCell != entityNode.Hexagon)
                    {
                        float bodyRotation;
                        if (newCell == entityNode.Hexagon.GetNeighbor(Hexagon.Direction.NorthEast))
                        {
                            bodyRotation = 60f;
                        }
                        else if (newCell == entityNode.Hexagon.GetNeighbor(Hexagon.Direction.SouthEast))
                        {
                            bodyRotation = 120f;
                        }
                        else if (newCell == entityNode.Hexagon.GetNeighbor(Hexagon.Direction.South))
                        {
                            bodyRotation = 180f;
                        }
                        else if (newCell == entityNode.Hexagon.GetNeighbor(Hexagon.Direction.SouthWest))
                        {
                            bodyRotation = 240f;
                        }
                        else if (newCell == entityNode.Hexagon.GetNeighbor(Hexagon.Direction.NorthWest))
                        {
                            bodyRotation = 300f;
                        }
                        else
                        {
                            bodyRotation = 0f;
                        }

                        state.Unit.RemainingRange -= 1;
                        
                        entityNode.Hexagon = newCell;
                        entityNode.UpdatePosition();

                        if (state.Unit.Body is not null)
                        {
                            state.Unit.Body.GlobalRotationDegrees = bodyRotation;
                        }

                        if (state.Unit.Weapon is not null)
                        {
                            state.Unit.Weapon.GlobalRotationDegrees = bodyRotation;
                        }
                    }

                    ChangeState(state with { Path = state.Path.Skip(1).ToList() });
                }
                else
                {
                    ChangeState(new GameState.Selected(entityNode.Hexagon, state.Unit));
                }
                break;
            case GameState.Attacking state:
                HexagonNode2D attackerEntity = state.Attacker.GetParent<HexagonNode2D>();
                if (Map is null)
                {
                    ChangeState(new GameState.Selected(attackerEntity.Hexagon, state.Attacker));
                    break;
                }

                HexagonNode2D? targetNode = Map.GetNodesAtCell(state.Target).FirstOrDefault(n => n.GetChildren().OfType<Unit>().Any());
                if (targetNode is null)
                {
                    ChangeState(new GameState.Selected(attackerEntity.Hexagon, state.Attacker));
                    break;
                }

                Unit targetUnit = targetNode.GetChildren().OfType<Unit>().First();
                
                int damage = state.Attacker.Damage - targetUnit.Armor;
                targetUnit.Integrity -= damage;
                float angle = GetAngleBetweenPositions(attackerEntity.Hexagon, state.Target);

                if (state.Attacker.Weapon is not null)
                {
                    state.Attacker.Weapon.GlobalRotationDegrees = angle;
                }
                else if (state.Attacker.Body is not null)
                {
                    state.Attacker.Body.GlobalRotationDegrees = angle;
                }

                state.Attacker.RemainingAttacks -= 1;

                if (targetUnit.Integrity <= 0)
                {
                    targetNode.QueueFree();
                }
                ChangeState(new GameState.Selected(attackerEntity.Hexagon, state.Attacker));
                break;
            case GameState.Startup:
                if (Players.Count >= 1)
                {
                    ChangeState(new GameState.NewRound());
                }
                break;
        }
        base._PhysicsProcess(delta);
    }

    private bool DoesCellHaveUnits(Hexagon cell)
    {
        return Map is not null && Map.GetNodesAtCell(cell).SelectMany(n => n.GetChildren()).OfType<Unit>().Any();
    }

    private List<Hexagon> FindPath(Hexagon start, Hexagon target)
        {
            if (Map is null) return [];

            if (DoesCellHaveUnits(target))
            {
                return [];
            }

            PriorityQueue<Vector2I, int> frontier = new();

            Vector2I startInOffsetCoords = start.GetOffsetPosition();
            Vector2I targetInOffsetCoords = target.GetOffsetPosition();
            frontier.Enqueue(startInOffsetCoords, 0);
            System.Collections.Generic.Dictionary<Vector2I, Vector2I?> cameFrom = [];
            System.Collections.Generic.Dictionary<Vector2I, int> costSoFar = [];
            cameFrom[startInOffsetCoords] = null;
            costSoFar[startInOffsetCoords] = 0;

            while (frontier.Count > 0)
            {
                Vector2I current = frontier.Dequeue();
                if (current == targetInOffsetCoords)
                {
                    frontier.Clear();
                }
                else
                {
                    foreach (Vector2I neighbour in Map.GetSurroundingCells(current))
                    {
                        Hexagon axialPosition = Hexagon.AtOffsetPosition(neighbour);
                        if (DoesCellHaveUnits(axialPosition))
                        {
                            continue;
                        }

                        int newCost = costSoFar[current] + 1;
                        if (costSoFar.TryGetValue(neighbour, out int value) && value <= newCost)
                        {
                            continue;
                        }

                        costSoFar[neighbour] = newCost;
                        int priority = (int)(newCost + current.DistanceTo(targetInOffsetCoords));
                        frontier.Enqueue(neighbour, priority);
                        cameFrom[neighbour] = current;
                    }
                }
            }

            List<Hexagon> path = [];
            {
                if (!cameFrom.TryGetValue(targetInOffsetCoords, out Vector2I? current) || !current.HasValue)
                {
                    return path;
                }

                path.Insert(0, target);
                while (current != startInOffsetCoords && current.HasValue)
                {
                    path.Insert(0, Hexagon.AtOffsetPosition(current.Value));
                    current = cameFrom[current.Value];
                }
            }

            return path;
        }

    private float GetAngleBetweenPositions(Hexagon first, Hexagon second)
    {
        Vector2 direction = first.Get2DPosition(cellSize).DirectionTo(second.Get2DPosition(cellSize));
        float radToDeg = Mathf.RadToDeg(direction.Angle());
        return radToDeg + 90; // The calculated angle is off by 90° from what we need
    }
    
    public bool CanAttackCellWithUnit(Unit unit, Hexagon targetInAxial)
    {
        HexagonNode2D? unitEntity = unit.GetParentOrNull<HexagonNode2D>();
        if (Map is null || unitEntity is null) return false;
        
        Player? unitPlayer = unitEntity.GetChildren().OfType<Player>().SingleOrDefault();
        if (unitPlayer is null)
        {
            return false;
        }

        int distanceToUnit = targetInAxial.DistanceTo(unitEntity.Hexagon);
        bool doesSelectedUnitBelongToCurrentPlayer = unitPlayer.PlayerId == CurrentPlayer;
        if (unit.RemainingAttacks <= 0 || !doesSelectedUnitBelongToCurrentPlayer || distanceToUnit < unit.MinAttackRange ||
            distanceToUnit > unit.MaxAttackRange)
        {
            return false;
        }

        if (Map.GetNodesAtCell(targetInAxial).Where(n => n.GetChildren().OfType<Unit>().Any()).Any(n =>
                n.GetChildren().OfType<Player>().Any(p => p.PlayerId == unitPlayer.PlayerId)))
        {
            return false;
        }

        if (unit.IsAttackIndirect)
        {
            Node2D? weaponOrBody = unit.Weapon ?? unit.Body;
            
            if (weaponOrBody is null) return false;
            float angle = GetAngleBetweenPositions(unitEntity.Hexagon, targetInAxial);
            return Mathf.Abs(angle - weaponOrBody.GlobalRotationDegrees) <= unit.IndirectWeaponAngleDegrees + 1; // Adjust for cells that would be slightly out of range, but look like they should be attackable
        }

        Vector2 adjustmentVector = new(0, cellSize / 2);
                
        PhysicsRayQueryParameters2D queryParameters = new();
        Vector2 unitPosition = unitEntity.Hexagon.Get2DPosition(cellSize);
        Vector2 targetPosition = targetInAxial.Get2DPosition(cellSize);
        float directionTo = (unitPosition + adjustmentVector).AngleToPoint(targetPosition);
        adjustmentVector = adjustmentVector.Rotated(directionTo);

        queryParameters.From = unitPosition + adjustmentVector;
        queryParameters.To = targetPosition;
        queryParameters.CollisionMask = 1u << UnitMask;
        queryParameters.CollideWithAreas = true;
        queryParameters.HitFromInside = false;

        Dictionary result = GetWorld2D().DirectSpaceState.IntersectRay(queryParameters);
        if (result.Count == 0 || IsCollidedObjectAtTarget(result))
        {
            return true;
        }
                
        queryParameters.From = unitPosition - adjustmentVector;
        result = GetWorld2D().DirectSpaceState.IntersectRay(queryParameters);
        
        return result.Count == 0 || IsCollidedObjectAtTarget(result);

        bool IsCollidedObjectAtTarget(Dictionary dictionary)
        {
            ulong hitId = dictionary["collider_id"].AsUInt64();
            if (InstanceFromId(hitId) is Node2D hitNode)
            {
                return Hexagon.At2DPosition(hitNode.GlobalPosition, cellSize) == targetInAxial;                
            }
            return false;
        }
    }
    
    /// <inheritdoc />
    public override void _UnhandledInput(InputEvent @event)
    {
        if (GameState is GameState.ContextMenu) return;
        
        switch (@event)
        {
            case InputEventMouse when Map is null:
                return;
            case InputEventMouseMotion mouseMotion:
            {
                QueueRedraw();
                if (Map.HoverLayer is null) return;
                Vector2I offsetPosition = Map.LocalToMap(Map.GetLocalMousePosition());
                
                Map.HoverLayer.Clear();
            
                if (Map.GetCellTileData(offsetPosition) is not null)
                {
                    Map.HoverLayer.SetCell(offsetPosition, 0, Vector2I.Zero, 1);
                }

                if (mouseMotion.ButtonMask == MouseButtonMask.Middle && Camera is not null)
                {
                    Camera.Position -= mouseMotion.Relative;
                }
                
                break;
            }
            case InputEventMouseButton button:
                if (button is { ButtonIndex: MouseButton.Left, Pressed: true } or { ButtonIndex: MouseButton.Right, Pressed: true })
                {
                    Vector2I position = Map.LocalToMap(Map.GetLocalMousePosition());
                    Hexagon hexagonCell = Hexagon.AtOffsetPosition(position);
                    

                    List<MenuItem> menuItems = GetMenuItemsForCellAndCurrentState(hexagonCell);
                    if (menuItems.Count > 0 && button.ButtonMask == MouseButtonMask.Left)
                    {
                        menuItems = SelectMostFittingActionItems(menuItems, Map.GetNodesAtCell(position));
                        
                        if (menuItems.Count == 1)
                        {
                            menuItems.First().Command.Invoke();
                        }
                        else
                        {
                            ShowContextMenu(menuItems.ToList(), GetScreenPosition(hexagonCell));
                        }
                    }
                    else
                    {
                        ShowContextMenu(menuItems.ToList(), GetScreenPosition(hexagonCell));
                    }
                    
                }
                else if (button.ButtonIndex == MouseButton.Right)
                {
                    ClearSelection();
                }
                else
                {
                    Vector2 zoomSpeed = new(0.05f, 0.05f);
                    switch (button.ButtonIndex)
                    {
                        case MouseButton.WheelUp when Camera is not null && Camera.Zoom.Length() <= 2:
                            Camera.Zoom += zoomSpeed;
                            break;
                        case MouseButton.WheelDown when Camera is not null:
                            Camera.Zoom -= zoomSpeed;
                            break;
                    }
                }
                break;
        }

        return;

        Vector2 GetScreenPosition(Hexagon hexagonCell)
        {
            Vector2 globalCellPosition = Map.ToGlobal(Map.MapToLocal(hexagonCell.GetOffsetPosition()));
            if (Camera is null) return globalCellPosition;

            return (globalCellPosition - Camera.Position) * Camera.Zoom + Camera.GetViewportRect().Size / 2f;
        }

        void ShowContextMenu(List<MenuItem> menuItems, Vector2 position)
        {
            if (Map is null || Camera is null || MapUI is null) return;

            menuItems.Add(new MenuItem(
                "End Turn",
                () => ChangeState(new GameState.NewRound()),
                new ItemType.IconItem(NewRoundIconPath)
                ));
            
            MapUI.ShowContextMenu(menuItems, position, () => { ChangeState(new GameState.Waiting()); });
            GameState = new GameState.ContextMenu(GameState);
        }
    }

    private List<MenuItem> SelectMostFittingActionItems(List<MenuItem> menuItems, List<HexagonNode2D> nodes)
    {
        List<MenuItem> reducedItems = menuItems;
        List<Unit> units = nodes.SelectMany(n => n.GetChildren().OfType<Unit>()).ToList();

        if (units.Count > 0)
        {
            List<MenuItem> attackCellItems = menuItems.Where(i => i is AttackCellItem).ToList();
            if (attackCellItems.Count > 0)
            {
                reducedItems = attackCellItems;
            }
            else
            {
                List<MenuItem> selectUnitItems = menuItems.Where(i => i is SelectUnitItem).ToList();

                if (selectUnitItems.Count > 0)
                {
                    reducedItems = selectUnitItems;
                }
            }
        }
        else
        {
            List<MenuItem> moveUnitItems = menuItems.Where(i => i is MoveUnitItem).ToList();

            if (moveUnitItems.Count > 0)
            {
                reducedItems = moveUnitItems;
            }
        }

        return reducedItems;
    }

    private List<MenuItem> GetMenuItemsForCellAndCurrentState(Hexagon axialPosition)
    {
        if (Map is null || Camera is null || MapUI is null) return [];
        List<HexagonNode2D> nodesAtCell = Map.GetNodesAtCell(axialPosition);

        GameState.Selected? currentSelection = GameState switch
        {
            GameState.Selected state => state,
            _ => null,
        };

        List<MenuItem> menuItems = [];
        menuItems.AddRange(nodesAtCell.Select(node => node.GetChildren().OfType<Unit>().SingleOrDefault()).OfType<Unit>().Select(GetItemForUnit));

        if (currentSelection is null)
        {
            return menuItems;
        }

        Hexagon unitPosition = currentSelection.Cell;
        Player? playerOfUnit = currentSelection.SelectedUnit.GetParent()?.GetChildren().OfType<Player>().FirstOrDefault();
        bool isUnitOwnedByCurrentPlayer = playerOfUnit?.PlayerId == CurrentPlayer;
        List<Hexagon> path = FindPath(unitPosition, axialPosition);
        if (isUnitOwnedByCurrentPlayer && currentSelection.SelectedUnit.IsInMovementRange(path.Count))
        {
            menuItems.Add(
                new MoveUnitItem(
                    this, currentSelection.SelectedUnit, path)
            );
        }
        if (isUnitOwnedByCurrentPlayer && CanAttackCellWithUnit(currentSelection.SelectedUnit, axialPosition))
        {
            menuItems.Add(new AttackCellItem(this, currentSelection.SelectedUnit, axialPosition));
        }

        return menuItems;

        MenuItem GetItemForUnit(Unit unit)
        {
            return new SelectUnitItem(this, "Tank", axialPosition, unit, "uid://qjh3gq1srb8n");
        }
    }

    private void ClearSelection()
    {
        Map?.SelectionLayer?.Clear();
        Map?.MovementRangeLayer?.Clear();
        Map?.RemainingMovementLayer?.Clear();
        Map?.AttackableLayer?.Clear();
        Map?.AttackRangeLayer?.Clear();
    }
    
    private void SelectCell(Hexagon axialPosition)
    {
        if (Map?.SelectionLayer is null || Map.MovementRangeLayer is null || Map.RemainingMovementLayer is null || Map.AttackableLayer is null || Map.AttackRangeLayer is null) return;
        List<HexagonNode2D> nodesAtCell = Map.GetNodesAtCell(axialPosition);
        List<Unit> units = nodesAtCell.SelectMany(n => n.GetChildren().OfType<Unit>()).ToList();

        Vector2I offsetPosition = axialPosition.GetOffsetPosition();
                    
        if (Map.GetCellTileData(offsetPosition) is null)
        {
            return;
        }

        ClearSelection();
        Map.SelectionLayer.SetCell(offsetPosition, 0, Vector2I.Zero, 1);
        Unit? unit = units.FirstOrDefault();
        if (unit is null)
        {
            return;
        }

        HashSet<Vector2I> checkedCells = [];
        
        Queue<Vector2I> cellsToCheck = new(NeighboursInMovementOrAttackRange(offsetPosition));
        
        while (cellsToCheck.TryDequeue(out Vector2I cell))
        {
            if ( Map.GetCellTileData(cell) is null) continue;
            
            if (!checkedCells.Add(cell)) continue;
            foreach (Vector2I neighbour in NeighboursInMovementOrAttackRange(cell))
            {
                cellsToCheck.Enqueue(neighbour);
            }

            int distance = Hexagon.AtOffsetPosition(cell).DistanceTo(axialPosition);
            if (distance <= unit.MaxAttackRange && distance >= unit.MinAttackRange)
            {
                Map.AttackRangeLayer.SetCell(cell, 0, Vector2I.Zero, 1);
                if (unit.RemainingAttacks > 0)
                {
                    if (CanAttackCellWithUnit(unit, Hexagon.AtOffsetPosition(cell)))
                    {
                        Map.AttackableLayer.SetCell(cell, 0, Vector2I.Zero, 1);
                    }
                }
            }

            if (distance <= unit.Mobility)
            {
                Map.MovementRangeLayer.SetCell(cell, 0, Vector2I.Zero, 1);
            }

            Player playerOfUnit = unit.GetParent().GetChildren().OfType<Player>().Single();

            if (playerOfUnit.PlayerId == CurrentPlayer && unit.IsInMovementRange(FindPath(axialPosition, Hexagon.AtOffsetPosition(cell)).Count))
            {
                Map.RemainingMovementLayer.SetCell(cell, 0, Vector2I.Zero, 1);
            }
        }

        return;

        IEnumerable<Vector2I> NeighboursInMovementOrAttackRange(Vector2I cell)
        {
            return Map.GetSurroundingCells(cell).Where(c =>
            {
                int distance = Hexagon.AtOffsetPosition(c).DistanceTo(axialPosition);
                return  c != offsetPosition && (distance <= unit.MaxAttackRange || distance <= unit.Mobility);
            });            
        }

    }

    /// <inheritdoc />
    public override void _Draw()
    {
        if (Map is null) return;
        switch (GameState)
        {
            case GameState.Selected state:
            {
                
                if (state.SelectedUnit.Mobility <= 0) return; 
                Hexagon mouseCell = Hexagon.AtOffsetPosition(Map.LocalToMap(GetLocalMousePosition()));
                Vector2 lastPosition = state.SelectedUnit.GetParent<Node2D>().Position;
                List<Hexagon> path = FindPath(state.Cell, mouseCell);
                foreach (Vector2 cellPosition in path.Select(hexagon => Map.MapToLocal(hexagon.GetOffsetPosition())))
                {
                    DrawLine(lastPosition, cellPosition, Colors.Black);
                    lastPosition = cellPosition;
                }
                break;
            }
        }
    }
}
