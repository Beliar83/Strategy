using System;
using System.Collections.Generic;
using Strategy.Components;

namespace Strategy;

public abstract record ItemType
{
    public record Item : ItemType;

    public record IconItem(string IconPath) : ItemType;
}

public record MenuItem(string Label, Action Command, ItemType ItemType);


public record SelectUnitItem : MenuItem
{
    public SelectUnitItem(GameWorld gameWorld, string unitName, Hexagon cell, Unit unit, string iconPath) : base(unitName,  () => gameWorld.ChangeState(new GameState.Selected(cell, unit)), new ItemType.IconItem(iconPath))
    { }
}

public record AttackCellItem : MenuItem
{
    private const string AttackIconPath = "uid://d0l8r5tp3p233";

    public AttackCellItem(GameWorld gameWorld, Unit attacker, Hexagon targetCell) : base("Attack", () => gameWorld.ChangeState(new GameState.Attacking(attacker, targetCell)), new ItemType.IconItem(AttackIconPath))
    {}
}

public record MoveUnitItem : MenuItem
{
    private const string MoveIconPath = "uid://cc3a8cadi02fc";

    public MoveUnitItem(GameWorld gameWorld, Unit unitToMove, List<Hexagon> path) : base("Move", () => gameWorld.ChangeState(new GameState.Moving(unitToMove, path)), new ItemType.IconItem(MoveIconPath))
    {}
}
