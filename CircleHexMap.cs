using Godot;

namespace Strategy;

[GlobalClass]
[Tool]
public partial class CircleHexMap : HexMap
{
    [Export]
    public int MapRadius { get; set; }
}
