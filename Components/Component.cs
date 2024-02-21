using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Runtime.CompilerServices;
using Godot;

namespace Strategy.Components;

[GlobalClass]
public partial class Component : Node
{
    public GameWorld? gameWorld;
    public event ComponentPropertyChangedEventHandler? PropertyChanged;
    
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

        this.gameWorld = null;
    }

    /// <inheritdoc />
    public override string[] _GetConfigurationWarnings()
    {
        return gameWorld is null
            ? ["Parent needs to be a direct or indirect child of a GameWorld"]
            : Array.Empty<string>();
    }

    protected virtual void OnPropertyChanged(StringName? propertyName = null)
    {
        PropertyChanged?.Invoke(this, new ComponentPropertyChangedEventArgs(propertyName));
    }

    protected bool SetField<T>(ref T field, T value, StringName? propertyName = null)
    {
        if (EqualityComparer<T>.Default.Equals(field, value)) return false;
        field = value;
        OnPropertyChanged(propertyName);
        return true;
    }
}
