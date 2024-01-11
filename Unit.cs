using Godot;

namespace Strategy
{
	[Tool]
	public partial class Unit : FSharp.Unit.UnitNode
	{
		[Export((PropertyHint)35, "Node2D")]
		public new NodePath BodyNode
		{
			get => base.BodyNode;
			set => base.BodyNode = value;
		}
	}
}
