using System;
using Godot;

namespace Strategy;

public class Hexagon
{
    private int q;
    private int r;
    private int s;

    public int Q
    {
        get => q;
        set
        {
            q = value;
            s = CalculateAxis(q, r);
        }
    }

    public int R
    {
        get => r;
        set
        {
            r = value;
            q = CalculateAxis(r, s);
        }
    }

    public int S
    {
        get => s;
        set
        {
            s = value;
            r = CalculateAxis(s, q);
        }
    }

    public Hexagon()
    {
    }
    
    private Hexagon(int q, int r, int s)
    {
        Q = q;
        R = r;
        S = s;
    }

    public enum Direction
    {
        East,
        NorthEast,
        NorthWest,
        West,
        SouthWest,
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

    public static Hexagon Zero => new(0, 0, 0);
   

    public static Hexagon NewAxial(int q, int r) => new(q, r, CalculateAxis(q, r));

    public static Hexagon At2DPosition(Vector2 position, float cellSize)
    {
        float q =
            (MathF.Sqrt(3f) / 3f * position.X - 1f / 3f * position.Y)
            / cellSize;

        float r = (2f / 3f * position.Y) / cellSize;
        float s = -q - r;
        (int rx, int ry, int rz) = CubeRound(q, r, s);
        
        return new Hexagon(rx, ry, rz);
    }
    
    public Vector2 Get2DPosition(float cellSize)
    {
        float x =
            cellSize
            * (MathF.Sqrt(3f) * Q
            + MathF.Sqrt(3f) / 2f * R);

        var y = cellSize * (3f / 2f * R);

        return new Vector2(x, y);
    }

    public Vector2I GetOffsetPosition()
    {
        var col = Q;
        var row = R + (Q + (Q & 1)) / 2;
        return new Vector2I(col, row);
    }
    
    public Hexagon FromVector2(Vector2 vector) => NewAxial((int)vector.X, (int)vector.Y);

    public void MoveQ(int length)
    {
        Q += length;
        S -= length;
    }

    public void MoveR(int length)
    {
        R += length;
        S -= length;
    }

    public void MoveS(int length)
    {
        Q -= length;
        S += length;
    }

    public int DistanceTo(Hexagon other)
    {
        // https://www.redblobgames.com/grids/hexagons/#distances-cube
        return (Math.Abs(Q - other.Q) + Math.Abs(R - other.R) + Math.Abs(S - other.S)) / 2;
    }

    public bool IsNeighbor(Hexagon other) => DistanceTo(other) == 1;

    public Hexagon GetNeighbor(Direction direction)
    {
        return direction switch
        {
            Direction.East => new Hexagon(Q + 1, R, S - 1),
            Direction.NorthEast => new Hexagon(Q + 1, R - 1, S),
            Direction.NorthWest => new Hexagon(Q, R - 1, S + 1),
            Direction.West => new Hexagon(Q - 1, R, S + 1),
            Direction.SouthWest => new Hexagon(Q - 1, R + 1, S),
            Direction.SouthEast => new Hexagon(Q, R + 1, S - 1),
            _ => throw new ArgumentOutOfRangeException(nameof(direction), direction, null),
        };
    }
}
