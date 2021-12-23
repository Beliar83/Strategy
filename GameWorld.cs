using Godot;

namespace Strategy
{
    public class GameWorld : FSharp.GameWorld.GameWorld
    {
        [Export]
        public new NodePath MapUI
        {
            get => base.MapUI;
            set => base.MapUI = value;
        }
    }
}
