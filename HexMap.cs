using Godot;
using Godot.Collections;

namespace Strategy
{
    [Tool]
    public partial class HexMap : FSharp.HexMap.HexMap
    {
        [Export]
        public new Array<Vector2> Cells
        {
            get => base.Cells;
            set => base.Cells = value;
        }
    }
}
