using Godot;

namespace Strategy;

[GlobalClass]
[Tool]
public abstract partial class Component : Resource
{
    public Entity? Entity { get; set; }
    public abstract object? GetValue();
}
