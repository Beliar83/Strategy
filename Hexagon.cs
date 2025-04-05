using System;
using Godot;

namespace Strategy;


public record Hexagon(int Q, int R)
{

    public int S => CalculateAxis(Q, R);
    
    public enum Direction
    {
        NorthEast,
        North,
        NorthWest,
        SouthWest,
        South,
        SouthEast,
    }

    private static (int rx, int ry, int rz) CubeRound(float x, float y, float z)
    {
        float rx = MathF.Round(x);
        float ry = MathF.Round(y);
        float rz = MathF.Round(z);

        float xDiff = Math.Abs(rx - x);
        float yDiff = Math.Abs(ry - y);
        float zDiff = Math.Abs(rz - z);

        if (xDiff > yDiff && xDiff > zDiff)
        {
            rx = -ry - rz;
        }
        else if (yDiff > zDiff)
        {
            ry = -rx - rz;
        }
        else
        {
            rz = -rx - ry;
        }

        return ((int)rx, (int)ry, (int)rz);
    }

    private static int CalculateAxis(int axis1, int axis2) => -axis1 - axis2;

    public static Hexagon Zero => new(0, 0);

    public static Hexagon At2DPosition(Vector2 position, float cellSize)
    {
        float q = 2f / 3 * position.X / cellSize;
        float r = (-1f / 3 * position.X + Mathf.Sqrt(3) / 3f * position.Y) / cellSize;
        
        float s = -q - r;
        (int rx, int ry, int _) = CubeRound(q, r, s);
        
        return new Hexagon(rx, ry);
    }

    public static Hexagon AtOffsetPosition(Vector2I position)
    {
        return new Hexagon(position.X, position.Y - (position.X + (position.X & 1)) / 2);
    }

    /// <inheritdoc />
    public override string ToString()
    {
        return $"q: {Q}, r: {R}, s: {S}";
    }

    public Vector2 Get2DPosition(float cellSize)
    {
        float x = cellSize * (3f / 2f * Q);

        float y = cellSize * (MathF.Sqrt(3f) / 2 * Q + Mathf.Sqrt(3f) * R);

        return new Vector2(x, y);
    }

    public Vector2I GetOffsetPosition()
    {
        int col = Q;
        int row = R + (Q + (Q & 1)) / 2;
        return new Vector2I(col, row);
    }
    
    public Hexagon FromVector2(Vector2 vector) => new((int)vector.X, (int)vector.Y);

    public Hexagon MoveQ(int length)
    {
        return new Hexagon(Q + length, R - length);
    }

    public Hexagon MoveR(int length)
    {
        return this with { R = R + length };
    }

    public Hexagon MoveS(int length)
    {
        return this with { Q = Q - length };
    }

    public int DistanceTo(Hexagon other)
    {
        // https://www.redblobgames.com/grids/hexagons/#distances-axial
        return (Math.Abs(Q - other.Q) + Math.Abs(Q + R - other.Q - other.R) + Math.Abs(R - other.R)) / 2;
    }

    public bool IsNeighbor(Hexagon other) => DistanceTo(other) == 1;
    
    public Hexagon GetNeighbor(Direction direction)
    {
        return direction switch
        {
            Direction.North => this with { R = R - 1 },
            Direction.NorthEast => new Hexagon(Q + 1, R - 1),
            Direction.SouthEast => this with { Q = Q + 1 },
            Direction.South => this with { R = R + 1 },
            Direction.SouthWest => new Hexagon(Q - 1, R + 1),
            Direction.NorthWest => this with { Q = Q - 1 },
            _ => throw new ArgumentOutOfRangeException(nameof(direction), direction, null),
        };
    }
}
