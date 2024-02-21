using Godot;

namespace Strategy;

[GlobalClass]
[Tool]
public partial class PlayerData : Resource
{
    [Export] public StringName Name { get; set; } = new();
    
    [Export]
    public Color Color { get; set; }
}
 
