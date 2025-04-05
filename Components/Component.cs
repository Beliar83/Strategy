using System;
using System.Collections.Generic;
using System.Linq;
using Godot;

namespace Strategy.Components;

[GlobalClass]
public abstract partial class Component : Node
{
    public GameWorld? gameWorld;
    public event ComponentPropertyChangedEventHandler? PropertyChanged;
    
    private static readonly Dictionary<Type, StringName> ComponentNames = new();

    /// <inheritdoc />
    public override void _EnterTree()
    {
        Type componentType = GetType();
        if (!ComponentNames.TryGetValue(componentType, out StringName? componentName))
        {
            componentName = $"{componentType.Name}Component";
            ComponentNames[componentType] = componentName;
        }

        AddToGroup(componentName);
        base._EnterTree();
    }
    
    /// <inheritdoc />
    public override void _ExitTree()
    {
        if (ComponentNames.TryGetValue(GetType(), out StringName? componentName))
        {
            RemoveFromGroup(componentName);
        }
        base._ExitTree();
    }

    public static IEnumerable<T> GetInstancesOfComponent<T>(SceneTree sceneTree) where T : Component
    {
        return ComponentNames.TryGetValue(typeof(T), out StringName? componentName) 
            ? sceneTree.GetNodesInGroup(componentName).OfType<T>() 
            : [];
    }
    
    
    /// <inheritdoc />
    public override void _Notification(int what)
    {
        if (what != NotificationParented && what != NotificationUnparented)
        {
            return;
        }

        Node? parent = GetParent()?.GetParent();
        while (parent is not null)
        {
            // ReSharper disable once LocalVariableHidesMember
            if (parent is GameWorld gameWorld)
            {
                this.gameWorld = gameWorld;
                return;
            }
                
            parent = parent.GetParent();
        }
        
        UpdateConfigurationWarnings();
        this.gameWorld = null;
    }

    /// <inheritdoc />
    public override string[] _GetConfigurationWarnings()
    {
        return gameWorld is null
            ? ["Parent needs to be a direct or indirect child of a GameWorld"]
            : [];
    }

    // ReSharper disable once VirtualMemberNeverOverridden.Global
    protected virtual void OnPropertyChanged(StringName? propertyName = null)
    {
        PropertyChanged?.Invoke(this, new ComponentPropertyChangedEventArgs(propertyName));
    }

    // ReSharper disable once UnusedMethodReturnValue.Global
    protected bool SetField<T>(ref T field, T value, StringName? propertyName = null)
    {
        if (EqualityComparer<T>.Default.Equals(field, value)) return false;
        field = value;
        OnPropertyChanged(propertyName);
        return true;
    }
}
