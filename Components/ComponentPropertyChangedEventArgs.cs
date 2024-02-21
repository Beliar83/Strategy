using System;
using Godot;

namespace Strategy.Components;

public class ComponentPropertyChangedEventArgs(StringName? propertyName) : EventArgs
{
    public virtual StringName? PropertyName { get; } = propertyName;
}
