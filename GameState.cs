using System.Collections.Generic;
using Strategy.Components;

namespace Strategy;

public abstract record GameState
{
    public record Startup : GameState;
    public record NewRound : GameState;
    public record Waiting : GameState;
    public record ContextMenu(GameState StoredState) : GameState;
    public record Selected(Hexagon Cell, Unit SelectedUnit) : GameState;
    public record Moving(Unit Unit, List<Hexagon> Path) : GameState;
    public record Attacking(Unit Attacker, Hexagon Target) : GameState;
}
