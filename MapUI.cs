using Godot;

namespace Strategy
{
	[Tool]
	public partial class MapUI : FSharp.MapUI.MapUI
	{
		[Export((PropertyHint)35, "Label")]
		public new NodePath PlayerLabel
		{
			get => base.PlayerLabel;
			set => base.PlayerLabel = value;
		}
	}
}
