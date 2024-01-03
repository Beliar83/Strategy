using Godot;

namespace Strategy
{
    public partial class GameWorld : FSharp.GameWorld.GameWorld
    {
        /// <inheritdoc />
        public override void _Ready()
        {
            base._Ready();
        }

        /// <inheritdoc />
        public override void _Process(double delta)
        {
            base._Process(delta);
        }

        /// <inheritdoc />
        public override void _PhysicsProcess(double delta)
        {
            base._PhysicsProcess(delta);
        }

        /// <inheritdoc />
        public override void _UnhandledInput(InputEvent @event)
        {
            base._UnhandledInput(@event);
        }

        /// <inheritdoc />
        public override void _Draw()
        {
            base._Draw();
        }

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
