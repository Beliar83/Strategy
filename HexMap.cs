using Godot;
using Godot.Collections;

namespace Strategy
{
	[Tool]
	[GlobalClass]
	public partial class HexMap : FSharp.HexMap.HexMap
	{
		[Export]
		public new int MapRadius
		{
			get => base.MapRadius;
			set => base.MapRadius = value;
		}
	}
}
