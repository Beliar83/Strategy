using Godot;

namespace Strategy;

[GlobalClass]
[Tool]
public partial class PlayerData : Resource
{
    [Export] public string Name { get; set; } = string.Empty;
    
    [Export]
    public Color Color { get; set; }
}
 
