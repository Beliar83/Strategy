using System.Linq;
using Godot;
using Godot.Collections;

namespace Strategy;

[Tool]
[GlobalClass]
public partial class HexMap : Node2D
{
	private int cellSize;
	
	[Export(PropertyHint.ResourceType, "HexagonMapType")]
	public HexagonMapType? MapType { get; set; }
	
	[Export]
	public int CellSize
	{
		get => cellSize;
		set
		{
			cellSize = value;
			foreach (HexagonNode2D hexagonNode2D in GetChildren().OfType<HexagonNode2D>())
			{
				hexagonNode2D.UpdatePosition();
			}
		}
	}
}