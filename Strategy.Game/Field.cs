using Microsoft.Xna.Framework;

namespace Strategy.Game;

public record struct Field(bool Movable, bool Attackable, Vector2 Position);