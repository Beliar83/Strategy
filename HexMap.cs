using Godot;
using Godot.Collections;

namespace Strategy
{
    [Tool]
    public class HexMap : FSharp.HexMap.HexMap
    {
        [Export]
        public new float CellSize
        {
            get => base.CellSize;
            set => base.CellSize = value;
        }

        [Export]
        public new Array<Vector2> Cells
        {
            get => base.Cells;
            set => base.Cells = value;
        }
    }
}
