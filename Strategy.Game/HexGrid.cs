using System.Collections.Immutable;
using Microsoft.Xna.Framework;

namespace Strategy.Game;

public record struct HexGrid(int CellSize)
{
    [Flags]
    enum CollisionMasks
    {
        Ground = 0,
        Unit = 1,
    }

    public static Hexagon[] CreateGrid(int radius)
    {
        var hexagons = new List<Hexagon>();

        for (int r = -radius; r <= radius; r++)
        {
            int qStart = Math.Max(-radius, -r - radius);
            int qEnd = Math.Min(radius, -r + radius);

            for (int q = qStart; q <= qEnd; q++)
            {
                var hexagon = Hexagon.NewAxial(q, r);
                if (hexagon.DistanceTo(Hexagon.Zero) < radius)
                {
                    hexagons.Add(hexagon);
                }
            }
        }

        return hexagons.ToArray();
    }

    public static Vector2 Get2DPositionOfHexagon(Hexagon hexagon, float cellSize)
    {
        float x = cellSize * (MathF.Sqrt(3f) * hexagon.Q + MathF.Sqrt(3f) / 2f * hexagon.R);
        float y = cellSize * (3f / 2f * hexagon.R);
        return new Vector2(x, y);
    }

    public static Hexagon[] GetNeighboursOfHexagon(Hexagon hexagon)
    {
        return new[]
        {
            hexagon.GetNeighbor(Hexagon.Direction.East),
            hexagon.GetNeighbor(Hexagon.Direction.NorthEast),
            hexagon.GetNeighbor(Hexagon.Direction.NorthWest),
            hexagon.GetNeighbor(Hexagon.Direction.West),
            hexagon.GetNeighbor(Hexagon.Direction.SouthWest),
            hexagon.GetNeighbor(Hexagon.Direction.SouthEast),
        };
    }

    public static float GetHexagonWidth(float cellSize) => MathF.Sqrt(3f) * cellSize;
    public static float GetHexagonHeight(float cellSize) => 2f * cellSize;
    public static float GetHalf(float value) => value / 2f;
    public static float GetQuarter(float value) => value / 4f;
    
    public static List<Vector2> GetHexagonPoints(float cellSize)
    {
        float width = GetHexagonWidth(cellSize);
        float height = GetHexagonHeight(cellSize);
        float halfHeight = GetHalf(height);
        float quarterHeight = GetQuarter(height);
        float halfWidth = GetHalf(width);
        return new List<Vector2>
        {
            new(-halfWidth, -quarterHeight),
            new(0f, -halfHeight),
            new(halfWidth, -quarterHeight),
            new(halfWidth, quarterHeight),
            new(0f, halfHeight),
            new(-halfWidth, quarterHeight),
            new(-halfWidth, -quarterHeight),
        };
    }

    public static ImmutableList<Vector2> GetHexagonTriangles(float cellSize)
    {
        List<Vector2> points = GetHexagonPoints(cellSize);
        points.Add(points[0]);
        
        Vector2 center = Vector2.Zero;

        return points
            .Zip(points.Skip(1), Tuple.Create)
            .SelectMany(p => new[] { center, p.Item1, p.Item2 })
            .ToImmutableList();
    }
}
