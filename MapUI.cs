using Godot;

namespace Strategy
{
	[Tool]
	public partial class MapUI : MarginContainer
	{
		[Export((PropertyHint)35, "Label")]
		public NodePath? PlayerLabel { get; set; }
	}
}
