using Godot;

namespace Strategy
{
    public class MapUI : FSharp.MapUI.MapUI
    {
        [Export((PropertyHint)35, "Label")]
        public new NodePath PlayerLabel
        {
            get => base.PlayerLabel;
            set => base.PlayerLabel = value;
        }

        [Export((PropertyHint)35, "Popup")]
        public new NodePath RadialMenu
        {
            get => base.RadialMenu;
            set => base.RadialMenu = value;
        }
    }
}
