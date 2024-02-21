using Godot;

namespace Strategy;

[GlobalClass]
[Tool]
public partial class CircleHexagonMapType : HexagonMapType
{
    [Export]
    public int MapRadius { get; set; }
}
