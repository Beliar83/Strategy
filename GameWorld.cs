using Godot;

namespace Strategy
{
    public class GameWorld : FSharp.GameWorld.GameWorld
    {
        [Export((PropertyHint)35, "MarginContainer")]
        public new NodePath MapUI
        {
            get => base.MapUI;
            set => base.MapUI = value;
        }

        [Export((PropertyHint)35, "Camera2D")]
        public new NodePath Camera
        {
            get => base.Camera;
            set => base.Camera = value;
        }
    }
}
